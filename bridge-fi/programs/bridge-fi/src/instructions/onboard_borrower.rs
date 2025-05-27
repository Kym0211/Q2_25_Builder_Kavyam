use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use crate::state::BorrowerProfile;

#[derive(Accounts)]
pub struct OnboardBorrower<'info> {
    #[account(mut)]
    pub borrower: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        init,
        payer = borrower,
        seeds = [b"borrower", borrower.key().as_ref()],
        bump,
        space = 8 + BorrowerProfile::INIT_SPACE
    )]
    pub borrower_profile: Account<'info, BorrowerProfile>,

    #[account(
        init,
        payer = borrower,
        associated_token::mint = mint_x,
        associated_token::authority = borrower
    )]
    pub borrower_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = borrower,
        associated_token::mint = mint_y,
        associated_token::authority = borrower
    )]
    pub borrower_y: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> OnboardBorrower<'info> {
    pub fn onboard_borrower(
        &mut self,
        bumps: &OnboardBorrowerBumps
    ) -> Result<()> {
        self.borrower_profile.set_inner(BorrowerProfile { 
            wallet: self.borrower.key(), 
            total_loans: 0,
            active_loans: 0, 
            kyc_verified: false,
            credit_score: 0,
            bump:  bumps.borrower_profile
        });

        Ok(())
    }
}