use anchor_lang::prelude::*;

use crate::{error::ReserveError, state::ReserveState};

#[derive(Accounts)]
pub struct ProcessLoanActivity<'info> {
    #[account(mut)]
    pub reserve: Account<'info, ReserveState>,
    pub authority: Signer<'info>,
}

impl<'info> ProcessLoanActivity<'info> {
    pub fn process_loan_activity(&mut self, borrow_amount: u64, is_repayment: bool) -> Result<()> {
        let reserve = &mut self.reserve;

        if is_repayment {
            reserve
                .total_borrows
                .checked_sub(borrow_amount)
                .ok_or(ReserveError::MathUnderflow)?;
        } else {
            reserve
                .total_borrows
                .checked_add(borrow_amount)
                .ok_or(ReserveError::MathOverflow)?;
        }

        //update utilization rate
        reserve.utilization_rate = if reserve.total_deposits > 0 {
            (reserve.total_borrows as u128)
                .checked_mul(10000)
                .and_then(|v| v.checked_div(reserve.total_deposits as u128))
                .map(|v| v as u16)
                .unwrap_or(0)
        } else {0};

        Ok(())
    }
}
