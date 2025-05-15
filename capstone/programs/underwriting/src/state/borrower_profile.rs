use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct BorrowerProfile {
    pub wallet: Pubkey,
    pub credit_score: u16,
    pub debt_to_income: u8,
    pub kyc_status: u8,
    pub risk_tiers: u8,
    pub max_loan_amount: u64,
    pub collateral_ratio: u16,
    pub soulbound_nft: Pubkey,
    pub risk_model: Pubkey
}
