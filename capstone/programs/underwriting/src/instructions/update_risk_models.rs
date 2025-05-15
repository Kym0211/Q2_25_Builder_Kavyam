use anchor_lang::prelude::*;

use crate::state::{RiskModel, RiskTier};

#[derive(Accounts)]
pub struct UpdateRiskModel<'info> {
    #[account(
        mut,
        seeds = [b"risk-model", authority.key().as_ref()],
        bump = risk_model.bump,
        has_one = authority
    )]
    pub risk_model: Account<'info, RiskModel>,

    #[account(mut)]
    pub authority: Signer<'info>
}

impl<'info> UpdateRiskModel<'info> {
    pub fn update_risk_model(
        &mut self,
        new_tiers: Vec<RiskTier>
    ) -> Result<()> {

        require!(
            new_tiers.len() <= RiskModel::MAX_TIERS,
            crate::error::UnderwritingError::InsufficientCreditScore
        );

        let risk_model = &mut self.risk_model;
        risk_model.tiers = new_tiers;

        Ok(())
    }
}