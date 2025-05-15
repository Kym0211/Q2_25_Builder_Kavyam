use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LenderPosition {
    pub depositor: Pubkey,
    pub lending_market: Pubkey,
    pub deposit_amount: u64,
    pub tokens_minted: u64,
    pub deposit_date: i64,
    pub last_interest_claim: i64,
}