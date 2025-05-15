use anchor_lang::prelude::*;
use crate::state::LendingMarket;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 +  // Discriminator
           32 +  // owner: Pubkey
           1 +   // bump: u8
           2 +   // fee_basis_points: u16
           8 +   // min_collateral_ratio: u64
           4 + (32 * 1) +  // risk_tiers: Vec<u8> with max 32 elements
           (4 * 2) +       // utilization_curve: [u16; 4]
           8 +   // creation_timestamp: i64
           8 +   // total_deposits: u64
           8,    // total_borrows: u64
        seeds = [b"lending-market", owner.key().as_ref()],
        bump
    )]
    pub lending_market: Account<'info, LendingMarket>,

    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        fee_basis_points: u16,
        min_collateral_ratio: u64,
        risk_tiers: Vec<u8>,
        utilization_curve: [u16; 4],
    ) -> Result<()> {
        
        let lending_market = &mut self.lending_market;
        lending_market.owner = self.owner.key();
        lending_market.fee_basis_points = fee_basis_points;
        lending_market.min_collateral_ratio = min_collateral_ratio;
        lending_market.risk_tiers = risk_tiers;
        lending_market.utilization_curve = utilization_curve;
        lending_market.creation_timestamp = Clock::get()?.unix_timestamp;
        lending_market.total_deposits = 0;
        lending_market.total_borrows = 0;

        Ok(())
    }
}