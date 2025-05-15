use anchor_lang::prelude::*;

#[error_code]
pub enum UnderwritingError {
    #[msg("KYC verification required")]
    KycNotVerified,
    
    #[msg("Insufficient credit score")]
    InsufficientCreditScore,
    
    #[msg("Loan amount exceeds limit")]
    LoanLimitExceeded,
    
    #[msg("Invalid KYC provider")]
    InvalidKycProvider,
    
    #[msg("Math overflow")]
    MathOverflow,
    
    #[msg("Math underflow")]
    MathUnderflow,
    
    #[msg("Failed to mint soulbound NFT")]
    NftMintFailed,
}