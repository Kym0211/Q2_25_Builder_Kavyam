use anchor_lang::prelude::*;

#[account]
pub struct LoanAccount {
    pub seed: u64,
    pub borrower: Pubkey,
    pub lender: Pubkey,
    pub principal: u64,
    pub interest_rate: u16,
    pub start_date: i64,
    pub due_date: i64,
    pub repaid_amount: u64,
    pub status: LoanStatus,
    pub bump: u8
}

impl LoanAccount {
    pub const INIT_SPACE: usize = 8 + 8 + 32 + 32 + 8 + 2 + 8 + 8 + 8 + 1;
}

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone)]
pub enum LoanStatus {
    Requested,
    Approved,
    Funded,
    Repaid,
    Defaulted,
}