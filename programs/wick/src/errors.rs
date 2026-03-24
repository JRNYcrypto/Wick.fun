use anchor_lang::prelude::*;

#[error_code]
pub enum WickError {
    #[msg("Vault timer has not elapsed yet")]
    TimerNotElapsed,
    #[msg("Vault balance is below the minimum threshold")]
    InsufficientBalance,
    #[msg("Retention ratio must be between 0 and 10000 basis points")]
    InvalidRetentionRatio,
    #[msg("Interval must be at least 60 seconds")]
    IntervalTooShort,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Unauthorized vault operation")]
    Unauthorized,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Buy transaction failed")]
    BuyFailed,
    #[msg("Sell transaction failed")]
    SellFailed,
}
