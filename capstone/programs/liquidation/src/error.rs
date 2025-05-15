use anchor_lang::prelude::*;

#[error_code]
pub enum LiquidationError {
    #[msg("Loan position is healthy")]
    HealthyPosition,
    
    #[msg("Price feed is stale")]
    StalePrice,
    
    #[msg("Price feed ID does not match reserve's registered price feed ID")]
    InvalidPriceFeed,
    
    #[msg("Price status is not trading")]
    PriceNotTrading,
    
    #[msg("Price conversion error")]
    PriceConversionError,
    
    #[msg("Math overflow")]
    MathOverflow,
    
    #[msg("Math underflow")]
    MathUnderflow,
    
    #[msg("Invalid liquidation parameters")]
    InvalidParameters,
    
    #[msg("Liquidation amount below minimum threshold")]
    BelowMinimumLiquidation,
    
    #[msg("Liquidation exceeds maximum portion of position")]
    ExceedsMaxLiquidationPortion,
}