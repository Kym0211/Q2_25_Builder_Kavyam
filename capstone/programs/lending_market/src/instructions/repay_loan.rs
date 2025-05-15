use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};

use crate::{error::LendingError, state::{LendingMarket, LoanObligation}};

#[derive(Accounts)]
pub struct RepayLoan<'info> {
    #[account(mut)]
    pub lending_market: Account<'info, LendingMarket>,

    #[account(mut)]
    pub liquidity_pool: Account<'info, TokenAccount>,

    #[account(
        mut,
        has_one = borrower
    )]
    pub loan_obligation: Account<'info, LoanObligation>,

    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(mut)]
    pub source_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> RepayLoan<'info> {
    pub fn repay_loan(
        &mut self,
        amount: u64
    ) -> Result<()> {

        let lending_market =&mut self.lending_market;
        let loan_obligation = &mut self.loan_obligation;

        //transfer repayment
        let cpi_accounts = Transfer{
            from: self.source_token_account.to_account_info(),
            to: self.liquidity_pool.to_account_info(),
            authority: lending_market.to_account_info()
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        let _ = transfer(cpi_context, amount)?;

        //update loan status
        loan_obligation.repaid_amount = loan_obligation
            .repaid_amount
            .checked_add(amount)
            .ok_or(LendingError::Overflow)?;

        if loan_obligation.repaid_amount >= loan_obligation.loan_amount {
            loan_obligation.status = LoanStatus::Repaid as u8;
        }

        //update market totals
        lending_market
            .total_borrows
            .checked_sub(amount)
            .ok_or(LendingError::Underflow)?;

        lending_market
            .total_deposits
            .checked_add(amount)
            .ok_or(LendingError::Overflow)?;

        Ok(())
    }
}


#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Clone)]
pub enum LoanStatus {
    Active,
    Repaid,
    Defaulted,
}