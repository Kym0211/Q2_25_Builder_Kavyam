#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("BPWKPL7uX4zWkm5FcJgbzFPaHJrck8vjfAkNhRwx6DA2");

pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fees: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.initialize(seed, fees, authority, &ctx.bumps)?;

        Ok(())
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)?;

        Ok(())
    }

}
