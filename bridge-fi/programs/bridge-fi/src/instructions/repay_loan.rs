use std::cmp::min;

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked}};

use crate::{error::LendingError, state::{loan_account::LoanStatus, LoanAccount}};

#[derive(Accounts)]
pub struct RepayLoan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    /// CHECK: The lender is validated via the `loan_account` constraint (`has_one = lender`)
    pub lender: AccountInfo<'info>,
    pub mint_x: Account<'info, Mint>,

    #[account(
        mut,
        has_one = lender,
        has_one = borrower,
        seeds = [b"loan-account", loan_account.seed.to_le_bytes().as_ref(), borrower.key().as_ref()],
        bump = loan_account.bump,
    )]
    pub loan_account: Account<'info, LoanAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = lender,
    )]
    pub lender_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = borrower,
    )]
    pub borrower_x: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>
}

impl<'info> RepayLoan<'info> {
    pub fn repay_loan(&mut self, amount: u64) -> Result<()> {
        let loan_account = &mut self.loan_account;
        let now = Clock::get()?.unix_timestamp;

        require!(now <= loan_account.due_date, LendingError::Overflow);
        require!(loan_account.status != LoanStatus::Repaid, LendingError::Overflow);

        // require!(now <= loan_account.due_date, "Due Date has Passed");

        let due_amount = loan_account.principal.checked_sub(loan_account.repaid_amount).unwrap();
        let repay_amount = min(amount, due_amount);

        let cpi_accounts = TransferChecked {
            from: self.borrower_x.to_account_info(),
            mint: self.mint_x.to_account_info(),
            to: self.lender_x.to_account_info(),
            authority: self.borrower.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer_checked(cpi_ctx, repay_amount, self.mint_x.decimals)?;

        loan_account.repaid_amount = loan_account.repaid_amount + repay_amount;

        // ✅ Mark repaid if complete
        if loan_account.repaid_amount >= loan_account.principal {
            loan_account.status = LoanStatus::Repaid;
        }

        Ok(())
    }
}