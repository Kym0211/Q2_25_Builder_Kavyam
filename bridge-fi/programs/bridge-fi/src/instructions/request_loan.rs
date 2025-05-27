use std::cmp::max;

use anchor_lang::prelude::*;

use crate::state::{loan_account::LoanStatus, LoanAccount, VaultState};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct RequestLoan<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        init,
        payer = borrower,
        space = 8 + LoanAccount::INIT_SPACE,
        seeds = [b"loan-account", seed.to_le_bytes().as_ref(), borrower.key().as_ref()],
        bump,
    )]
    pub loan_account: Account<'info , LoanAccount>,

    #[account(
        mut,
        seeds = [b"vault-state", vault_state.admin.key().as_ref()],
        bump = vault_state.bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>
}

impl<'info> RequestLoan<'info> {
    pub fn request_loan(
        &mut self,
        seed: u64,
        amount: u64,
        max_interest_rate: u16,
        due_date: i64,
        bumps: &RequestLoanBumps
    ) -> Result<()> {
        let interest_rate = max(self.calculate_interest_rate().unwrap(), max_interest_rate);
        self.loan_account.set_inner(LoanAccount { 
            seed, 
            borrower: self.borrower.key(), 
            lender: Pubkey::default(), 
            principal: amount, 
            interest_rate, 
            start_date: 0, 
            due_date, 
            repaid_amount: 0, 
            status: LoanStatus::Requested,
            bump: bumps.loan_account
        });

        Ok(())
    }

    pub fn check_utilization_rate(&self, total_borrowed: u64, total_deposited: u64) -> Option<u64> {
        // Scale up to avoid too much precision loss
        let scaled_borrowed = total_borrowed.checked_mul(1_000)?;
        scaled_borrowed.checked_div(total_deposited)
    }
    
    pub fn calculate_interest_rate(&self) -> Result<u16> {
        let total_borrowed = self.vault_state.total_borrowed;
        let total_deposited = self.vault_state.total_supplied;
    
        // Calculate utilization rate (scaled by 1,000 for some precision)
        let utilization_rate = self.check_utilization_rate(total_borrowed, total_deposited).unwrap();
    
        let base_rate = 5_000; // 5% scaled by 1,000
        let interest_rate = utilization_rate.checked_add(base_rate).unwrap();
    
        // Convert back to percentage (remove scaling if needed, but here we keep as is for example)
        // Since we scaled by 1,000, and base_rate is 5,000 (5%), add scaled utilization
        // To get a percentage as u16, divide by 10 (if you want 1 decimal place) or use as is for scaled integer
        // Here, just return the sum (scaled by 1,000) as u16, which may truncate if > u16::MAX!
        // But u16::MAX is 65,535, so if your rate is < 65.535% scaled by 1,000, it's safe.
        // If you want to return as a true percentage, divide by 10 (for 1 decimal place) or 100 (for integer %)
        // For this example, we'll assume you want the scaled value as u16 (beware of truncation!):
        Ok(interest_rate as u16)
    }
    
}