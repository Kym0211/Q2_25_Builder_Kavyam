use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};

use crate::{error::LendingError, state::{LendingMarket, LoanObligation}};


#[derive(Accounts)]
pub struct ProcessLoan<'info> {
    #[account(mut)]
    pub lending_market: Account<'info, LendingMarket>,

    #[account(mut)]
    pub liquidity_pool: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = borrower,
        space = 8 + LoanObligation::INIT_SPACE,
        seeds = [
            b"loan-obligation",
            borrower.key().as_ref(),
            lending_market.key().as_ref()
        ],
        bump
    )]
    pub loan_obligation: Account<'info, LoanObligation>,

    #[account(mut)]
    pub borrower: Account<'info, TokenAccount>,

    #[account(mut)]
    pub borrower_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> ProcessLoan<'info> {
    pub fn process_loan(
        &mut self,
        loan_amount: u64,
        interest_rate: u16,
        duration: u64
    ) -> Result<()> {

        let lending_market =&self.lending_market;

        //check available liquidity
        let available_liquidity = lending_market
            .total_deposits
            .checked_sub(lending_market.total_borrows)
            .ok_or(LendingError::InsufficientLiquidity)?;
        
        if available_liquidity < loan_amount {
            return Err(LendingError::InsufficientLiquidity.into());
        }

        //transfer funds to borrower
        let seeds = &[
            b"lending-market",
            lending_market.owner.as_ref(),
            &[lending_market.bump],
        ];

        let signer_seeds = [&seeds[..]];

        let cpi_accounts = Transfer {
            from: self.liquidity_pool.to_account_info(),
            to: self.borrower.to_account_info(),
            authority: self.lending_market.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            cpi_accounts, 
            &signer_seeds
        );

        let _ = transfer(cpi_context, loan_amount)?;

        //initialize loan obligation
        self.loan_obligation.set_inner(LoanObligation { 
            borrower: self.borrower.key(), 
            lending_market: self.lending_market.key(), 
            loan_amount, 
            interest_rate, 
            origination_date: Clock::get()?.unix_timestamp, 
            duration, 
            repaid_amount: 0,
            status: LoanStatus::Active as u8, 
            next_payment_due: Clock::get()?.unix_timestamp.checked_add(duration as i64).ok_or(LendingError::Overflow)?
        });

        //update market totals
        self.lending_market.total_borrows = lending_market
            .total_borrows
            .checked_add(loan_amount)
            .ok_or(LendingError::Overflow)?;


        Ok(())
    }
}


#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Clone)]
pub enum LoanStatus {
    Active,
    Repaid,
    Defaulted,
}