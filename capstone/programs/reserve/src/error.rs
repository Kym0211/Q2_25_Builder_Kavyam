use anchor_lang::prelude::*;

#[error_code]
pub enum ReserveError {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Math underflow")]
    MathUnderflow,
    #[msg("Insufficient insurance coverage")]
    InsufficientInsurance,
    #[msg("Invalid reserve parameters")]
    InvalidParameters,
}
