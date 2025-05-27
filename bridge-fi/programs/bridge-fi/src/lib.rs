#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("4Ezz1RyF5YWboqCTr5UrxTnWoArXY94JXHbbS1SpxTsx");

mod instructions;
mod state;
mod error;

use instructions::*;


#[program]
pub mod bridge_fi {
    use super::*;

    pub fn onboard_borrower(ctx: Context<OnboardBorrower>) -> Result<()> {
        ctx.accounts.onboard_borrower(&ctx.bumps)?;

        Ok(())
    }

    pub fn verify_kyc(ctx: Context<VerifyKYC>) -> Result<()> {
        ctx.accounts.verify_kyc()?;

        Ok(())
    }

    pub fn onboard_lender(ctx: Context<OnboardLender>) -> Result<()> {
        ctx.accounts.onboard_lender(&ctx.bumps)?;

        Ok(())
    }

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.initialize_vault(&ctx.bumps)?;

        Ok(())
    }

    pub fn initialize_reserve(ctx: Context<InitializeReserve>) -> Result<()> {
        ctx.accounts.initialize_reserve()?;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;

        Ok(())
    }

    pub fn request_loan(
        ctx: Context<RequestLoan>,
        seed: u64,
        amount: u64,
        max_interest_rate: u16,
        due_date: i64,
    ) -> Result<()> {
        ctx.accounts.request_loan(seed, amount, max_interest_rate, due_date, &ctx.bumps)?;

        Ok(())
    }

    pub fn approve_and_fund_loan(ctx: Context<ApproveAndFundLoan>) -> Result<()> {
        ctx.accounts.approve_and_fund_loan()?;

        Ok(())
    }

    pub fn repay_loan(ctx: Context<RepayLoan>, amount: u64) -> Result<()> {
        ctx.accounts.repay_loan(amount)?;

        Ok(())
    }

    pub fn close_loan(ctx: Context<CloseLoan>) -> Result<()> {
        ctx.accounts.close_loan()?;

        Ok(())
    }
}
