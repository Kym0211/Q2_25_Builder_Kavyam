use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u16, //basispoints
    pub bump: u8,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    pub name: String
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + (3*1) + 4 + 32;
}