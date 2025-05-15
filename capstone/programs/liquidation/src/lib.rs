#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;


mod instructions;
mod state;
mod error;

use instructions::*;

declare_id!("5mpktkWnqXD89hDDVGczSHytVuWXyTAinKNW8KjjVGJB");

#[program]
pub mod lending_market {
    use super::*;

    pub fn liquidate_loan(ctx: Context<LiquidateLoan>, max_debt_to_repay: u64) -> Result<()> {
        ctx.accounts.liquidate_loan(max_debt_to_repay)
    }
}
