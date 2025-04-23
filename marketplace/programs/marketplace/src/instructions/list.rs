use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::MasterEditionAccount, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{state::Marketplace, Listing};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        seeds = [b"marketplace", name.as_str().as_ref()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
        space = Listing::INIT_SPACE
    )]
    pub listing: Account<'info, Listing>,

    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [
            b"metadata",
            metadata.program.key().as_ref(),
            maker_mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, Metadata>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}