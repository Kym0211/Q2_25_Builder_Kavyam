use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, transfer, MintTo, Token, TokenAccount, Transfer};

use crate::{error::ReserveError, state::ReserveState};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub reserve: Account<'info, ReserveState>,

    #[account(mut)]
    pub liquidity_pool: Account<'info, TokenAccount>,

    #[account(mut)]
    pub source: Account<'info, TokenAccount>,

    #[account(mut)]
    pub deposit_certificate: Account<'info, TokenAccount>,

    #[account(mut)]
    pub liquidity_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub depositor: Signer<'info>,

    pub token_program: Program<'info, Token>
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let reserve = &mut self.reserve;

        //transfer tokens from lender
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.source.to_account_info(),
            to: self.liquidity_pool.to_account_info(),
            authority: self.depositor.to_account_info()
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        let _ = transfer(cpi_context, amount)?;

        //update reserve state
        reserve
            .total_deposits
            .checked_add(amount)
            .ok_or(ReserveError::MathOverflow)?;

        //mint liquidity tokens
        let mint_amount = amount
            .checked_mul(10u64.pow(reserve.decimals.into()))
            .ok_or(ReserveError::MathOverflow)?;

        mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(), 
                MintTo {
                    mint: self.liquidity_token.to_account_info(),
                    to: self.deposit_certificate.to_account_info(),
                    authority: reserve.to_account_info()
                }, 
                &[&[
                    b"reserve",
                    reserve.owner.as_ref(),
                    &[reserve.bump]
                ]]
            ),
            mint_amount
        )?;

        Ok(())
    }
}