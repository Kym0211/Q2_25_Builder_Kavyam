use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LenderProfile {
    pub wallet: Pubkey,
    pub total_lended_amount: u64,
    pub active_loans: u64,
    pub bump: u8
}