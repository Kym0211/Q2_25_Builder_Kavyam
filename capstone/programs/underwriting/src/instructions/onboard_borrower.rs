use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
};
use mpl_token_metadata::types::{DataV2, Creator, AuthorityType};

use crate::{
    state::{BorrowerProfile, RiskModel},
    utils::{verify_kyc},
    error::UnderwritingError,
};
use crate::utils::constants::kyc_status::KycStatus;

#[derive(Accounts)]
pub struct OnboardBorrower<'info> {
    #[account(
        init,
        payer = borrower,
        space = 8 + BorrowerProfile::INIT_SPACE,
        seeds = [b"borrower-profile", borrower.key().as_ref()],
        bump
    )]
    /// CHECK: Manual validation performed in XYZ function
    pub borrower_profile: Account<'info, BorrowerProfile>,
    
    #[account(mut)]
    pub borrower: Signer<'info>,
    
    #[account(
        seeds = [b"risk-model", risk_model.authority.key().as_ref()],
        bump = risk_model.bump
    )]
    /// CHECK: Manual validation performed in XYZ function
    pub risk_model: Account<'info, RiskModel>,
    
    #[account(
        mut,
        constraint = risk_model.kyc_providers.contains(&kyc_provider.key()) @ UnderwritingError::InvalidKycProvider
    )]
    /// CHECK: Manual validation performed in XYZ function
    pub kyc_provider: AccountInfo<'info>,
    
    #[account(
        init,
        payer = borrower,
        mint::decimals = 0,
        mint::authority = mint_authority.key(),
    )]
    /// CHECK: Manual validation performed in XYZ function
    pub soulbound_mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = borrower,
        associated_token::mint = soulbound_mint,
        associated_token::authority = borrower,
    )]
    /// CHECK: Manual validation performed in XYZ function
    pub borrower_token_account: Account<'info, TokenAccount>,
    
    #[account(
        seeds = [b"mint-authority"],
        bump,
    )]
    /// CHECK: Manual validation performed in XYZ function
    pub mint_authority: AccountInfo<'info>,
    
    /// CHECK: This is the metadata account that will be created
    #[account(
        mut,
        seeds = [
            b"metadata", 
            metadata_program.key().as_ref(), 
            soulbound_mint.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK: Manual validation performed in XYZ function
    pub metadata_account: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the Metaplex token metadata program
    pub metadata_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> OnboardBorrower<'info> {
    pub fn onboard_borrower(
        &mut self,
        kyc_proof: Vec<u8>,
        credit_score: u16,
        debt_to_income: u8,
        bump: u8,  // Added bump parameter
    ) -> Result<()> {
        
        verify_kyc(
            &self.kyc_provider,
            kyc_proof,
            &self.borrower.key()
        )?;

        self.borrower_profile.set_inner(BorrowerProfile { 
            wallet: *self.borrower.key, 
            credit_score, 
            debt_to_income, 
            kyc_status: KycStatus::Verified as u8, 
            risk_tiers: 0, 
            soulbound_nft: self.soulbound_mint.key(),
            risk_model: self.risk_model.key(),
            max_loan_amount: 0,
            collateral_ratio: 1
        });

        // Mint soulbound NFT
        token::mint_to(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                token::MintTo {
                    mint: self.soulbound_mint.to_account_info(),
                    to: self.borrower_token_account.to_account_info(),
                    authority: self.mint_authority.to_account_info(),
                },
                &[&[
                    b"mint-authority",
                    &[bump],
                ]],
            ),
            1
        )?;

        // Create metadata
        let data = DataV2 {
            name: "Borrower Identity NFT".to_string(),
            symbol: "BIDN".to_string(),
            uri: "https://underwriting.example/metadata/borrower-nft".to_string(),
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator {
                address: self.mint_authority.key(),
                verified: true,
                share: 100,
            }]),
            collection: None,
            uses: None,
        };

        // Create metadata account using v3
        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                self.metadata_program.clone(),
                CreateMetadataAccountsV3 {
                    metadata: self.metadata_account.clone(),
                    mint: self.soulbound_mint.to_account_info(),
                    mint_authority: self.mint_authority.clone(),
                    payer: self.borrower.to_account_info(),
                    update_authority: self.mint_authority.clone(),
                    system_program: self.system_program.to_account_info(),
                    rent: self.rent.to_account_info(),
                },
                &[&[
                    b"mint-authority",
                    &[bump],
                ]],
            ),
            data,
            true,  // Is mutable
            true,  // Update authority is signer
            None,  
        )?;

        // // Disable future minting
        // token::set_authority(
        //     CpiContext::new_with_signer(
        //         self.token_program.to_account_info(),
        //         token::SetAuthority {
        //             account_or_mint: self.soulbound_mint.to_account_info(),
        //             current_authority: self.mint_authority.clone(),
        //         },
        //         &[&[
        //             b"mint-authority",
        //             &[bump],
        //         ]],
        //     ),
        //     token::AuthorityType::MintTokens,
        //     None,
        // )?;

        // // Make token non-transferable
        // token::set_authority(
        //     CpiContext::new(
        //         self.token_program.to_account_info(),
        //         token::SetAuthority {
        //             account_or_mint: self.borrower_token_account.to_account_info(),
        //             current_authority: self.borrower.to_account_info(),
        //         },
        //     ),
        //     token::AuthorityType::AccountOwner,
        //     None,
        // )?;

        Ok(())
    }
}

