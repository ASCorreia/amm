use anchor_lang::error_code;

#[error_code]
pub enum AmmError {
    #[msg("Default Error")]
    DefaultError,
    #[msg("Fee is bigger than 100")]
    InvalidFee,
    #[msg("Invalid authority")]
    InvalidAuthority,
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            _ => AmmError::DefaultError,
        }
    }
}