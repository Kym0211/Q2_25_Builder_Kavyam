use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BorrowerProfile {
    pub wallet: Pubkey,
    pub total_loans: u64,
    pub active_loans: u64,
    pub kyc_verified: bool,
    pub credit_score: u64,
    pub bump: u8
}