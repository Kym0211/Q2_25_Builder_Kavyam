use anchor_lang::prelude::*;

use crate::state::{RiskModel, RiskTier};

#[derive(Accounts)]
pub struct InitializeRiskModel<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + RiskModel::INIT_SPACE,
        seeds = [b"risk-model", authority.key().as_ref()],
        bump
    )]
    pub risk_model: Account<'info, RiskModel>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeRiskModel<'info> {    
    pub fn initialize(&mut self, tiers: Vec<RiskTier>, kyc_providers: Vec<Pubkey>, bumps: u8) -> Result<()> {
    
        // Verify tiers length doesn't exceed maximum
        require!(
            tiers.len() <= RiskModel::MAX_TIERS,
            crate::error::UnderwritingError::InsufficientCreditScore
        );
        
        // Verify KYC providers length doesn't exceed maximum
        require!(
            kyc_providers.len() <= RiskModel::MAX_KYC_PROVIDERS,
            crate::error::UnderwritingError::InvalidKycProvider
        );
        
        self.risk_model.set_inner(RiskModel { 
            authority: *self.authority.key, 
            tiers: tiers, 
            kyc_providers: kyc_providers, 
            bump: bumps
        });
        
        Ok(())
    }
    
}