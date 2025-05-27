use anchor_lang::prelude::*;

use crate::state::BorrowerProfile;

#[derive(Accounts)]
pub struct VerifyKYC<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,

    #[account(
        mut,
        seeds = [b"borrower", borrower.key().as_ref()],
        bump = borrower_profile.bump,
    )]
    pub borrower_profile: Account<'info, BorrowerProfile>,
}

impl<'info> VerifyKYC<'info> {
    pub fn verify_kyc(
        &mut self,
    ) -> Result<()> {
        // Add KYC verification logic using oracle e.g chainlink

        // for MVP
        self.borrower_profile.kyc_verified = true;
        Ok(())
    }
}