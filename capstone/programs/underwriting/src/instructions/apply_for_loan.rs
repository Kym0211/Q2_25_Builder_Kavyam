use anchor_lang::prelude::*;

use crate::{
    error::UnderwritingError, state::{BorrowerProfile, RiskModel, RiskTier}, utils::calculate_max_loan
};
use crate::utils::constants::kyc_status::KycStatus;

#[derive(Accounts)]
pub struct ApplyForLoan<'info> {
    #[account(mut, has_one = risk_model)]
    pub borrower_profile: Account<'info, BorrowerProfile>,
    
    #[account(
        seeds = [b"risk-model", risk_model.authority.key().as_ref()],
        bump = risk_model.bump
    )]
    pub risk_model: Account<'info, RiskModel>,
    
    #[account(
        mut,
        constraint = borrower_profile.wallet == borrower.key() @ UnderwritingError::InvalidKycProvider
    )]
    pub borrower: Signer<'info>,
}

impl<'info> ApplyForLoan<'info> {
    pub fn apply_for_loan(&mut self, amount: u64) -> Result<()> {
        let borrower = &self.borrower_profile;
        let risk_model = &self.risk_model;

        require!(
            borrower.kyc_status == KycStatus::Verified as u8,
            UnderwritingError::KycNotVerified
        );

        let mut matching_tiers: Vec<&RiskTier> = risk_model
            .tiers
            .iter()
            .filter(|t| borrower.credit_score >= t.min_score)
            .collect();

        matching_tiers.sort_by(|a, b| b.tier_id.cmp(&a.tier_id));

        let tier = matching_tiers
        .first()
        .ok_or(UnderwritingError::InsufficientCreditScore)?;

        let max_loan = calculate_max_loan(borrower.debt_to_income, tier.max_ltv)?;
        require!(amount <= max_loan, UnderwritingError::LoanLimitExceeded);

        let borrower_profile = &mut self.borrower_profile;
        borrower_profile.risk_tiers = tier.tier_id;
        borrower_profile.collateral_ratio = tier.collateral_ratio;
        borrower_profile.max_loan_amount = max_loan;

        Ok(())
    }
}