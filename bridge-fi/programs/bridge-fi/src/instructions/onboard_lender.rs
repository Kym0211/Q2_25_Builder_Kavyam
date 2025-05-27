use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::state::LenderProfile;

#[derive(Accounts)]
pub struct OnboardLender<'info> {
    #[account(mut)]
    pub lender: Signer<'info>,
    pub mint_x: Account<'info, Mint>,

    #[account(
        init,
        payer = lender,
        seeds = [b"lender", lender.key().as_ref()],
        bump,
        space = 8 + LenderProfile::INIT_SPACE
    )]
    pub lender_profile: Account<'info, LenderProfile>,

    #[account(
        init,
        payer = lender,
        associated_token::mint = mint_x,
        associated_token::authority = lender
    )]
    pub lender_x: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> OnboardLender<'info> {
    pub fn onboard_lender(
        &mut self,
        bumps: &OnboardLenderBumps
    ) -> Result<()> {
        self.lender_profile.set_inner(LenderProfile { 
            wallet: self.lender.key(), 
            total_lended_amount: 0,
            active_loans: 0,
            bump:  bumps.lender_profile,
        });

        Ok(())
    }
}
