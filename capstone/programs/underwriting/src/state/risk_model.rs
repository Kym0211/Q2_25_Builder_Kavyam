use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, InitSpace)]
pub struct RiskTier {
    pub tier_id: u8,
    pub min_score: u16,
    pub max_ltv: u16,
    pub collateral_ratio: u16,
    pub interest_rate: u16
}

#[account]
#[derive(InitSpace, Default)]
pub struct RiskModel {
    pub authority: Pubkey,
    #[max_len(32)]
    pub tiers: Vec<RiskTier>,
    #[max_len(32)]
    pub kyc_providers: Vec<Pubkey>,
    pub bump: u8
}

impl RiskModel {
    pub const MAX_TIERS: usize = 10;
    pub const MAX_KYC_PROVIDERS: usize = 5;
}