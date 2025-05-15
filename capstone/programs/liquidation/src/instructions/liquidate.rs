use std::cmp::min;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{error::LiquidationError, state::{LoanObligation, ReserveState}};

#[derive(Accounts)]
pub struct LiquidateLoan<'info> {
    #[account(
        mut,
        has_one = reserve,
        constraint = loan_obligation.debt_amount > 0,
        constraint = loan_obligation.collateral_amount > 0
    )]
    pub loan_obligation: Box<Account<'info, LoanObligation>>,

    #[account(mut)]
    pub reserve: Box<Account<'info, ReserveState>>,

    #[account(
        mut,
        address = reserve.liquidity_pool,
        token::mint = loan_obligation.debt_mint
    )]
    pub liquidity_pool: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        address = reserve.collateral_pool,
        token::mint = loan_obligation.collateral_mint
    )]
    pub collateral_pool: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"reserve", reserve.key().as_ref()],
        bump = reserve.bump
    )]
    /// CHECK: Manual validation performed in XYZ function
    pub reserve_authority: AccountInfo<'info>,

    // The liquidator should NOT be constrained to be the borrower
    #[account(mut)]
    pub liquidator: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = loan_obligation.debt_mint,
        associated_token::authority = liquidator
    )]
    pub liquidator_debt_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = loan_obligation.collateral_mint,
        associated_token::authority = liquidator
    )]
    pub liquidator_collateral_account: Box<Account<'info, TokenAccount>>,

    #[account()]
    pub price_update: Account<'info, PriceUpdateV2>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>
}

#[event]
pub struct LiquidationEvent {
    pub loan: Pubkey,
    pub borrower: Pubkey,
    pub liquidator: Pubkey,
    pub debt_repaid: u64,
    pub collateral_liquidated: u64,
    pub timestamp: i64,
}

impl<'info> LiquidateLoan<'info> {
    pub fn liquidate_loan(&mut self, max_debt_to_repay: u64) -> Result<()> {
        let clock = Clock::get()?;

        let price_update = &self.price_update;
        
        // Verify the feed ID matches what's expected for this collateral
        let expected_feed_id = get_feed_id_from_hex(&self.reserve.price_feed_id)
            .map_err(|_| LiquidationError::InvalidPriceFeed)?;

        let maximum_age = 30;
        //2.  calculate collateral value

        let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &expected_feed_id)?;

        // Convert price to a decimal value based on the exponent
        let price_value = price.price as f64 * 10f64.powf(price.exponent as f64);
        
        // Apply the price to the collateral amount
        // We're using a f64 for the calculation, then converting back to u64
        let collateral_value = (self.loan_obligation.collateral_amount as f64 * price_value) as u64;

        // 3. check health factor
        let mut health_factor: f64 = 0.0;
        if self.loan_obligation.debt_amount == 0 {
            health_factor = f64::MAX;
        }else {
            let threshold_ratio = self.loan_obligation.liquidation_threshold as f64 / 10000.0; // Convert basis points to ratio
            let adjusted_collateral = collateral_value as f64 * threshold_ratio;
    
            let health_factor = adjusted_collateral / self.loan_obligation.debt_amount as f64;
    
            require!(
                health_factor < 1.0,
                LiquidationError::HealthyPosition
            );
        }
        
        // 4. Calculate liquidation amount

        // Step 1: Determine how much debt will be repaid
        let debt_to_repay = min(max_debt_to_repay, self.loan_obligation.debt_amount);
        
         // Enforce minimum liquidation size
         require!(
             debt_to_repay >= 100, // Example minimum value, adjust as needed
             LiquidationError::BelowMinimumLiquidation
         );
         
         
         // Calculate collateral equivalent (debt / price)
         let collateral_equivalent = (debt_to_repay as f64 / price_value) as u64;
         
         // Apply liquidation bonus
         let bonus_multiplier = 1.0 + (self.reserve.liquidation_bonus as f64 / 10000.0);
         let collateral_to_liquidate = (collateral_equivalent as f64 * bonus_multiplier) as u64;
         
         // Ensure we don't liquidate more than available collateral
         let final_collateral = min(collateral_to_liquidate, self.loan_obligation.collateral_amount);
         
         // Verify we don't exceed max portion of position (e.g., max 50%)
         let max_liquidation_portion = (self.loan_obligation.collateral_amount as f64 * 0.5) as u64; // 50% max
             
         require!(
             final_collateral <= max_liquidation_portion,
             LiquidationError::ExceedsMaxLiquidationPortion
         );

        let (final_debt_to_repay, final_collatera_to_liquidate) = (debt_to_repay, final_collateral);

        //5 Execute token transfers
        //transfer debt token from liquidator to protocol
        token::transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.liquidator_debt_account.to_account_info(),
                    to: self.liquidity_pool.to_account_info(),
                    authority: self.liquidator.to_account_info()
                }
            ), final_debt_to_repay
        )?;

        let seeds = &[
            b"reserve",
            self.reserve.to_account_info().key.as_ref(),
            &[self.reserve.bump],
        ];
        let signer = &[&seeds[..]];

        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.collateral_pool.to_account_info(),
                    to: self.liquidator_collateral_account.to_account_info(),
                    authority: self.reserve_authority.to_account_info(),
                },
                signer,
            ),
            collateral_to_liquidate,
        )?;

        //6. update loan and reserve state

        let obligation = &mut self.loan_obligation;

        let debt_repaid = final_debt_to_repay;

        let collateral_liquidated = collateral_to_liquidate;

        obligation.debt_amount = obligation.debt_amount
            .checked_sub(debt_repaid)
            .ok_or(LiquidationError::MathUnderflow)?;
        
        obligation.collateral_amount = obligation.collateral_amount
            .checked_sub(collateral_liquidated)
            .ok_or(LiquidationError::MathUnderflow)?;

        let reserve = &mut self.reserve;
        reserve.total_liquidations = reserve.total_liquidations
            .checked_add(1)
            .ok_or(LiquidationError::MathOverflow)?;
        
        // Track any bad debt (if collateral wasn't enough to cover the loan)
        // This implementation assumes normalized units; adjust as needed
        
        reserve.last_liquidation_timestamp = Clock::get()?.unix_timestamp;


        //emit an event
        emit!(LiquidationEvent {
            loan: self.loan_obligation.key(),
            borrower: self.loan_obligation.borrower,
            liquidator: self.liquidator.key(),
            debt_repaid: final_debt_to_repay,
            collateral_liquidated: collateral_to_liquidate,
            timestamp: clock.unix_timestamp,
        });


        Ok(())
    }
}

