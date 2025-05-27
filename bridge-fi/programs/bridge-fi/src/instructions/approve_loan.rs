use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked} 
};


use crate::state::{loan_account::LoanStatus, BorrowerProfile, LenderProfile, LoanAccount, VaultState};

#[derive(Accounts)]
pub struct ApproveAndFundLoan<'info> {
    #[account(mut)]
    pub lender: Signer<'info>,

    #[account(mut)]
    pub borrower: Signer<'info>,

    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"lender", lender.key().as_ref()],
        bump = lender_profile.bump,
    )]
    pub lender_profile: Account<'info, LenderProfile>,

    #[account(
        mut,
        seeds = [b"loan-account", loan_account.seed.to_le_bytes().as_ref(), borrower.key().as_ref()],
        bump = loan_account.bump,
    )]
    pub loan_account: Account<'info, LoanAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = borrower,
    )]
    pub borrower_x: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = borrower,
    )]
    pub borrower_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault-state", vault_state.admin.key().as_ref()],
        bump = vault_state.bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = vault_state,
    )]
    pub vault_x: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = vault_state,
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"borrower", borrower.key().as_ref()],
        bump = borrower_profile.bump,
    )]
    pub borrower_profile: Account<'info, BorrowerProfile>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> ApproveAndFundLoan<'info> {
    pub fn approve_and_fund_loan(&mut self) -> Result<()> {
        let loan_account = &mut self.loan_account;

        let clock = Clock::get()?;
        let start_date = clock.unix_timestamp;

        loan_account.lender = self.lender.key();
        loan_account.status = LoanStatus::Approved;
        loan_account.start_date = start_date;

        self.borrower_to_vault()?;
        self.vault_to_borrower()?;

        self.loan_account.status = LoanStatus::Funded;

        self.borrower_profile.total_loans += 1;
        self.borrower_profile.active_loans += 1;
        self.lender_profile.active_loans += 1;

        Ok(())
    }

    pub fn borrower_to_vault(&self) -> Result<()> {
        // Tokenise Real World Assets using oracle like DIA xREAL and deposit that in lenders atay


        // ToDo calculate collateral amount
        let cpi_accounts = TransferChecked {
            from: self.borrower_y.to_account_info(),
            mint: self.mint_y.to_account_info(),
            to: self.vault_y.to_account_info(),
            authority: self.borrower.to_account_info(),
        };

        // let value = self.loan_account.principal.checked_mul(10).unwrap();
        let value = self.loan_account.principal * 10;
        let collateral_value = value / 8;

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, collateral_value, self.mint_x.decimals)?;

        Ok(())
    }

    pub fn vault_to_borrower(&mut self) -> Result<()> {

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault-state",
            self.vault_state.admin.as_ref(),
            &[self.vault_state.bump]
        ]];

        let transfer_accounts = TransferChecked{ 
            from: self.vault_x.to_account_info(), 
            mint: self.mint_x.to_account_info(),
            to: self.borrower_x.to_account_info(), 
            authority: self.vault_state.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), transfer_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.loan_account.principal, self.mint_x.decimals)?;

        self.vault_state.total_supplied -= self.loan_account.principal;
        self.lender_profile.total_lended_amount -= self.loan_account.principal;

        Ok(())
    }
}