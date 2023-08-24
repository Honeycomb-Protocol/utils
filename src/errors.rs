use anchor_lang::prelude::error_code;

#[error_code]
pub enum UtilErrors {
    #[msg("Opertaion overflowed")]
    Overflow,

    #[msg("NFT validation failed")]
    InvalidNFT,

    #[msg("Invalid New Authority Record")]
    InvalidNewAuthorityRecord,
}
