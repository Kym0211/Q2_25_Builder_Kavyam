use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ReserveState {
    pub owner: Pubkey,
    pub bump: u8,
    pub decimals: u8,
    pub base_borrow_rate: u16,
    pub utilization_curve: [u16; 4],
    pub reserve_factor: u16,
    pub insurance_factor: u16,
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub utilization_rate: u16,
    pub insurance_total: u64,
    pub reserve_total: u64,
    pub last_updated: i64,
    pub insurance_fund: Pubkey,
    pub liquidity_pool: Pubkey
}