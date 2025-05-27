use anchor_lang::prelude::*;


#[error_code]
pub enum LendingError {
    #[msg("Invalid collateral ratio")]
    InvalidCollateralRatio,
    #[msg("Loan default detected")]
    LoanDefault,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Invalid loan status")]
    InvalidLoanStatus,
    #[msg("Arithmetic overflow")]
    Overflow,
    #[msg("Arithmetic underflow")]
    Underflow,
    #[msg("Calculation error")]
    CalculationError,
}