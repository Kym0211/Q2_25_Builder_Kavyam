use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::state::ReserveState;

#[derive(Accounts)]
pub struct InitializeReserve<'info>{
    #[account(mut)]
    pub admin: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        seeds = [b"reserve-state", admin.key().as_ref(), mint.key().as_ref()],
        bump,
        space = 8 + ReserveState::INIT_SPACE
    )]
    pub reserve_state: Account<'info, ReserveState>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint,
        associated_token::authority = reserve_state,
    )]
    pub reserve_vault: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> InitializeReserve<'info>  {
    pub fn initialize_reserve(&mut self) -> Result<()> {
        self.reserve_state.set_inner(ReserveState { 
            mint: self.mint.key(), 
            vault_bump: self.reserve_state.vault_bump, 
            state_bump: self.reserve_state.state_bump 
        });

        Ok(())
    }
}