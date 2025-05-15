#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod instructions;
mod state;
mod utils;
mod error;

use instructions::*;

pub use state::borrower_profile::BorrowerProfile;
pub use state::risk_model::{RiskModel, RiskTier};
pub use error::UnderwritingError;
declare_id!("AAPzJhMAm8Lqm1qMNQdgmqtA4cpUktWnuzAg8QQ7V8qs");

#[program]
pub mod underwriting_program {
    use super::*;

    // Initialize risk model with tiers
    pub fn initialize_risk_model(
        ctx: Context<InitializeRiskModel>,
        tiers: Vec<RiskTier>,
        kyc_providers: Vec<Pubkey>,
    ) -> Result<()> {

        let bump = ctx.bumps.risk_model;
        ctx.accounts.initialize(tiers, kyc_providers, bump)
    }

    // Onboard new borrower with KYC verification
    pub fn onboard_borrower(
        ctx: Context<OnboardBorrower>,
        kyc_proof: Vec<u8>,
        credit_score: u16,
        debt_to_income: u8,
    ) -> Result<()> {
        let bump = ctx.bumps.borrower_profile;
        ctx.accounts.onboard_borrower(kyc_proof, credit_score, debt_to_income, bump)
    }

    // Apply for loan with risk assessment
    pub fn apply_for_loan(ctx: Context<ApplyForLoan>, amount: u64) -> Result<()> {
        ctx.accounts.apply_for_loan(amount)
    }

    // Update risk tiers (admin only)
    pub fn update_risk_model(ctx: Context<UpdateRiskModel>, new_tiers: Vec<RiskTier>) -> Result<()> {
        ctx.accounts.update_risk_model(new_tiers)
    }
}