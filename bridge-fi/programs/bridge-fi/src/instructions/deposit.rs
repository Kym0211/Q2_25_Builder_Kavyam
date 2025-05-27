use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked}};

use crate::state::{LenderProfile, VaultState};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub lender: Signer<'info>,

    pub mint_x: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = lender,
    )]
    pub lender_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = vault_state
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"lender", lender.key().as_ref()],
        bump = lender_profile.bump,
    )]
    pub lender_profile: Account<'info, LenderProfile>,

    #[account(
        mut,
        seeds = [b"vault-state", vault_state.admin.key().as_ref()],
        bump = vault_state.bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.lender_x.to_account_info(),
            mint: self.mint_x.to_account_info(),
            to: self.vault_x.to_account_info(),
            authority: self.lender.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.mint_x.decimals)?;

        self.lender_profile.total_lended_amount += amount;
        self.vault_state.total_supplied += amount;

        Ok(())
    }
}