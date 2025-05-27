use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount}
};

use crate::state::VaultState;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,
        seeds = [b"vault-state", admin.key().as_ref()],
        bump,
        space = 8 + VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_x,
        associated_token::authority = vault_state
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = mint_y,
        associated_token::authority = vault_state
    )]
    pub vault_y: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeVault<'info> {
    pub fn initialize_vault(&mut self, bumps: &InitializeVaultBumps) -> Result<()> {
        self.vault_state.set_inner(VaultState { 
            admin: self.admin.key(), 
            mint_x: self.mint_x.key(), 
            mint_y: self.mint_y.key(),
            vault_x: self.vault_x.key(),
            vault_y: self.vault_y.key(),
            total_borrowed: 0,
            total_supplied: 0,
            bump: bumps.vault_state
        });

        Ok(())
    }
}