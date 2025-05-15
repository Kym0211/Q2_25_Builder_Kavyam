use std::cmp::min;

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};

use crate::{error::ReserveError, state::ReserveState};

#[derive(Accounts)]
pub struct ProcessLiquidationLoss<'info> {
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
    pub liquidity_pool: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>
}

impl<'info> ProcessLiquidationLoss<'info> {
    pub fn process_liquidation_loss(&mut self, loss_amount: u64) -> Result<()> {
        let reserve = &mut self.reserve;

        //check insurance coverage
        let coverage = min(loss_amount, reserve.insurance_total);

        //transfer from insurance fund
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.insurance_fund.to_account_info(),
            to: self.liquidity_pool.to_account_info(),
            authority: reserve.to_account_info()
        };

        let _ = transfer(
            CpiContext::new_with_signer(cpi_program, cpi_accounts, &[&[
                b"reserve",
                reserve.owner.as_ref(),
                &[reserve.bump]
            ]]),
            coverage
        );

        //update state
        reserve
            .insurance_total
            .checked_sub(coverage)
            .ok_or(ReserveError::MathUnderflow)?;

        reserve
            .total_borrows
            .checked_sub(loss_amount)
            .ok_or(ReserveError::MathUnderflow)?;
        
        Ok(())
    }
}