#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;


mod instructions;
mod state;
mod error;

use instructions::*;

declare_id!("XnZM1abpYCvCQCDfYATfHgnkF2xQF4bph5ThJYK26Wh");

#[program]
pub mod lending_market {
    use super::*;

    pub fn initialize( 
        ctx: Context<Initialize>,
        fee_basis_points: u16,
        min_collateral_ratio: u64,
        risk_tiers: Vec<u8>,
        utilization_curve: [u16; 4],
    ) -> Result<()> {
        ctx.accounts.initialize(fee_basis_points, min_collateral_ratio, risk_tiers, utilization_curve)
    }
    
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn process_loan(
        ctx: Context<ProcessLoan>, 
        loan_amount: u64,
        interest_rate: u16,
        duration: u64
    ) -> Result<()> {
        ctx.accounts.process_loan(loan_amount, interest_rate, duration)
    }

    pub fn repay_loan(
        ctx: Context<RepayLoan>, 
        amount: u64
    ) -> Result<()> {
        ctx.accounts.repay_loan(amount)
    }
}
