use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LoanObligation{
    pub borrower: Pubkey,
    pub lending_market: Pubkey,
    pub loan_amount: u64,
    pub interest_rate: u16,
    pub origination_date: i64,
    pub duration: u64,
    pub repaid_amount: u64,
    pub status: u8,
    pub next_payment_due: i64,
}