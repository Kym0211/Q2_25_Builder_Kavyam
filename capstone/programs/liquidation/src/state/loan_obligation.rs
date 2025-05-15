use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct LoanObligation {
    pub borrower: Pubkey,
    pub debt_mint: Pubkey,
    pub collateral_mint: Pubkey,
    pub debt_amount: u64,
    pub collateral_amount: u64,
    pub liquidation_threshold: u16,  // Basis points (e.g., 8000 = 80%)
    pub reserve: Pubkey,
    pub creation_timestamp: i64,
    pub last_update_timestamp: i64,
}