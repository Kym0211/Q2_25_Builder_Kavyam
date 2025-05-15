use anchor_lang::prelude::*;

use crate::state::ReserveState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + ReserveState::INIT_SPACE,
        seeds = [b"reserve", owner.key().as_ref()],
        bump
    )]
    pub reserve: Account<'info, ReserveState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn initialize( 
        &mut self,
        base_rate: u16,
        utilization_curve:[u16; 4],
        reserve_factor: u16,
        insurance_factor: u16,
        decimals: u8
    ) -> Result<()> {

        let reserve = &mut self.reserve;
        reserve.owner = self.owner.key();
        reserve.base_borrow_rate = base_rate;
        reserve.utilization_curve = utilization_curve;
        reserve.reserve_factor = reserve_factor;
        reserve.insurance_factor = insurance_factor;
        reserve.decimals = decimals;
        
        Ok(())
    }
}