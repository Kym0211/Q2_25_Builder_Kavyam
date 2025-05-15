use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LendingMarket {
    pub owner: Pubkey,
    pub bump: u8,
    pub fee_basis_points: u16,
    pub min_collateral_ratio: u64,
    #[max_len(3)]
    pub risk_tiers: Vec<u8>,
    pub utilization_curve: [u16; 4],
    pub creation_timestamp: i64,
    pub total_deposits: u64,
    pub total_borrows: u64,
}