use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};

use crate::{error::ReserveError, state::ReserveState};

#[derive(Accounts)]
pub struct ProcessFees<'info> {
    #[account(mut)]
    pub reserve: Account<'info, ReserveState>,

    #[account(
        mut,
        address = reserve.insurance_fund,
        seeds = [b"insurance", reserve.key().as_ref()],
        bump
    )]
    pub insurance_fund: Account<'info, TokenAccount>,

    #[account(mut)]
    pub fee_source: Account<'info, TokenAccount>,

    #[account(mut)]
    pub liquidity_pool: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ProcessFees<'info> {
    pub fn process_fees(&mut self, amount: u64) -> Result<()> {
        let reserve = &mut self.reserve;

        //calculate allocations
        let insurance_amount = amount
            .checked_mul(reserve.insurance_factor.into())
            .and_then(|v| v.checked_div(10000))
            .ok_or(ReserveError::MathOverflow)?;

        let reserve_amount = amount
            .checked_mul(reserve.reserve_factor.into())
            .and_then(|v| v.checked_div(10000))
            .ok_or(ReserveError::MathOverflow)?;

        //transfer to insurance fund
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer { 
            from: self.fee_source.to_account_info(), 
            to: self.insurance_fund.to_account_info(), 
            authority: reserve.to_account_info()
        };

        transfer(
        CpiContext::new_with_signer(
                cpi_program, 
                cpi_accounts, 
                &[&[
                    b"reserve",
                    reserve.owner.as_ref(),
                    &[reserve.bump]
                ]]
            ),
            insurance_amount
        )?;

        //update reserve state
        reserve
            .insurance_total
            .checked_add(insurance_amount)
            .ok_or(ReserveError::MathOverflow)?;

        reserve 
            .reserve_total
            .checked_add(reserve_amount)
            .ok_or(ReserveError::MathOverflow)?;

        Ok(())
    }
}