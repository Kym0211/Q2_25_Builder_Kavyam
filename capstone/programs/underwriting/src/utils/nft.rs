use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3},
    token::{self, Mint, Token, TokenAccount},
};
use mpl_token_metadata::types::{DataV2, Creator};

use crate::error::UnderwritingError;

pub fn mint_soulbound_nft(
    mint_authority: AccountInfo,  // Should be PDA signer
    mint: Account<Mint>,
    token_account: Account<TokenAccount>,
    payer: Signer,
    token_program: Program<Token>,
    metadata_program: AccountInfo,
    system_program: Program<System>,
    rent: Sysvar<Rent>,
    metadata_account: AccountInfo,  // Add metadata account as parameter
) -> Result<()> {
    // 1. Mint the initial token
    token::mint_to(
        CpiContext::new(
            token_program.to_account_info(),
            token::MintTo {
                mint: mint.to_account_info(),
                to: token_account.to_account_info(),
                authority: mint_authority.clone(),
            },
        ).with_signer(&[&[
            b"mint-authority",
            &[bump],  // Add actual bump here
        ]]),
        1,
    )?;

    // 2. Create metadata (Updated for v3)
    let data = DataV2 {
        name: "Borrower Identity NFT".to_string(),
        symbol: "BIDN".to_string(),
        uri: "https://underwriting.example/metadata/borrower-nft".to_string(),
        seller_fee_basis_points: 0,
        creators: Some(vec![Creator {
            address: mint_authority.key(),
            verified: true,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    // 3. Create metadata account using v3
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            metadata_program.clone(),
            CreateMetadataAccountsV3 {
                metadata: metadata_account.clone(),
                mint: mint.to_account_info(),
                mint_authority: mint_authority.clone(),
                payer: payer.to_account_info(),
                update_authority: mint_authority.clone(),
                system_program: system_program.to_account_info(),
                rent: rent.to_account_info(),
            },
            &[&[
                b"mint-authority",
                &[bump],  // Actual bump from PDA derivation
            ]],
        ),
        data,
        false, // Immutable metadata
        true,  // Update authority is signer
        None,  // No collection
        None,  // No token standard (required in v3)
    )?;

    // 4. Disable future minting
    token::set_authority(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            token::SetAuthority {
                account_or_mint: mint.to_account_info(),
                current_authority: mint_authority.clone(),
            },
            &[&[
                b"mint-authority",
                &[bump],  // Actual bump here
            ]],
        ),
        token::AuthorityType::MintTokens,
        None,
    )?;

    // 5. Make token non-transferable
    token::set_authority(
        CpiContext::new(
            token_program.to_account_info(),
            token::SetAuthority {
                account_or_mint: token_account.to_account_info(),
                current_authority: payer.to_account_info(),
            },
        ),
        token::AuthorityType::AccountOwner,
        None,
    )?;

    Ok(())
}
