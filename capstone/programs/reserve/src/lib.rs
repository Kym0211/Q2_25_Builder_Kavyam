#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
mod state;

mod error;
use instructions::*;
declare_id!("E1GYsEPRoPk7oCXhnCfBmEDpj6L6YYaeiz5ZWk8DapWW");

#[program]
pub mod reserve {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        base_rate: u16,
        utilization_curve:[u16; 4],
        reserve_factor: u16,
        insurance_factor: u16,
        decimals: u8
    ) -> Result<()> {
        ctx.accounts.initialize(base_rate, utilization_curve, reserve_factor, insurance_factor, decimals)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn process_loan_activity(ctx: Context<ProcessLoanActivity>, borrow_amount: u64, is_repayment: bool) -> Result<()> {
        ctx.accounts.process_loan_activity(borrow_amount, is_repayment)
    }

    pub fn process_fees(ctx: Context<ProcessFees>, amount: u64) -> Result<()> {
        ctx.accounts.process_fees(amount)
    }

    pub fn process_liquidation_loss(ctx: Context<ProcessLiquidationLoss>, loss_amount: u64) -> Result<()> {
        ctx.accounts.process_liquidation_loss(loss_amount)
    }
}
