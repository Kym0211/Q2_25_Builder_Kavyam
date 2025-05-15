use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount}, token::{Transfer, transfer}};

use crate::{error::LendingError, state::{LenderPosition, LendingMarket}};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
    )]
    pub lending_market: Account<'info, LendingMarket>,

    #[account(mut)]
    pub liquidity_pool: Account<'info, TokenAccount>,

    #[account(mut)]
    pub source_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = depositor,
        space = 8 + LenderPosition::INIT_SPACE,
        seeds = [
            b"lender-position",
            depositor.key().as_ref(),
            lending_market.key().as_ref()
        ],
        bump
    )]
    pub lender_position: Account<'info, LenderPosition>,

    #[account(mut)]
    depositor: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let lending_market = &mut self.lending_market;

        //Transfer Tokens from lender to lp
        let cpi_accounts = Transfer {
            from: self.source_token_account.to_account_info(),
            to: self.liquidity_pool.to_account_info(),
            authority: self.depositor.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        let _ = transfer(cpi_context, amount);

        //calculate and collect fees
        let fee = amount
            .checked_mul(lending_market.fee_basis_points.into())
            .and_then(|v| v.checked_div(10000))
            .ok_or(LendingError::CalculationError)?;

        let net_amount = amount.checked_sub(fee).ok_or(LendingError::CalculationError)?;

        //update lender position
        let lender_position = &mut self.lender_position;
        
        lender_position.deposit_amount = lender_position
            .deposit_amount
            .checked_add(net_amount)
            .ok_or(LendingError::Overflow)?;

        lender_position.tokens_minted = lender_position
            .tokens_minted
            .checked_add(net_amount)
            .ok_or(LendingError::Overflow)?;

        lender_position.deposit_date = Clock::get()?.unix_timestamp;

        // update market totals
        lending_market.total_deposits = lending_market
            .total_deposits
            .checked_add(net_amount)
            .ok_or(LendingError::Overflow)?;


        Ok(())
    }
}