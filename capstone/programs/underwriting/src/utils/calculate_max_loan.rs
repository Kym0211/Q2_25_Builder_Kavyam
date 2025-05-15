use anchor_lang::prelude::*;

use crate::error::UnderwritingError;

pub fn calculate_max_loan(dti: u8, ltv: u16) -> Result<u64> {
    let income: u64 = 100; // simplified - replace with actual income

    let max_debt = income.checked_mul(100).unwrap() / dti as u64;

    max_debt
        .checked_mul(ltv as u64)
        .map(|v| v / 100)
        .ok_or(UnderwritingError::MathOverflow.into())
}