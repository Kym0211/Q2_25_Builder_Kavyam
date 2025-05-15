use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct ReserveState {
    pub liquidity_pool: Pubkey,
    pub collateral_pool: Pubkey,
    pub liquidation_bonus: u16,          // Basis points (e.g., 500 = 5%)
    pub price_feed_id: String,           // Hex string of the Pyth price feed ID
    pub total_liquidations: u64,
    pub total_bad_debt: u64,
    pub last_liquidation_timestamp: i64,
    pub bump: u8,                        // Bump for PDA derivation
}