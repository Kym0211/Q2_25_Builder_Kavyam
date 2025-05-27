use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ReserveState{
    pub mint: Pubkey,
    pub vault_bump: u8,
    pub state_bump:u8
}