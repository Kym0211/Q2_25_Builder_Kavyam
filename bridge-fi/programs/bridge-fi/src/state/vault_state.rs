use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub admin: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub vault_x: Pubkey,
    pub vault_y: Pubkey,
    pub total_supplied: u64,
    pub total_borrowed: u64,
    pub bump: u8
}