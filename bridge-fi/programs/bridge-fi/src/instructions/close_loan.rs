use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::{error::LendingError, state::{loan_account::LoanStatus, BorrowerProfile, LenderProfile, LoanAccount}};

#[derive(Accounts)]
pub struct CloseLoan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    /// CHECK: The lender is validated via the `loan_account` constraint (`has_one = lender`)
    pub lender: AccountInfo<'info>,

    #[account(
        mut,
        has_one = lender,
        has_one = borrower,
        seeds = [b"loan-account", loan_account.seed.to_le_bytes().as_ref(), borrower.key().as_ref()],
        bump,
    )]
    pub loan_account: Account<'info, LoanAccount>,

    #[account(
        mut,
        seeds = [b"borrower", borrower.key().as_ref()],
        bump = borrower_profile.bump,
    )]
    pub borrower_profile: Account<'info, BorrowerProfile>,

    #[account(
        mut,
        seeds = [b"lender", lender.key().as_ref()],
        bump = lender_profile.bump,
    )]
    pub lender_profile: Account<'info, LenderProfile>,

    pub token_program: Program<'info, Token>,
}

impl<'info> CloseLoan<'info> {
    pub fn close_loan(&mut self) -> Result<()> {
        let loan = &mut self.loan_account;
        let now = Clock::get()?.unix_timestamp;

        require!(now > loan.due_date || loan.status == LoanStatus::Repaid, LendingError::Overflow);

        let total_due = loan.principal - loan.repaid_amount;

        if total_due == 0 {
            loan.status = LoanStatus::Repaid
        } else {
            loan.status = LoanStatus::Defaulted
        }

        self.borrower_profile.active_loans -= 1;
        self.lender_profile.active_loans -= 1;

        self.update_credit_score(total_due)?;


        Ok(())
    }

    pub fn update_credit_score(&mut self, _total_due: u64) -> Result<()> {
        // code to update credit_score

        // for demo we just updating it to 800
        self.borrower_profile.credit_score = 800;

        Ok(())
    }
}