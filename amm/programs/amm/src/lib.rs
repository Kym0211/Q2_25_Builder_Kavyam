#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("DG9KZ5sVEq7Mm1ksmDujAwicJ6t1nvdXSj59Zef7pdZy");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fees: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(seed, fees, authority, ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }
}