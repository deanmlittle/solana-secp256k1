use solana_program::secp256k1_recover::Secp256k1RecoverError;
// use solana_program::secp256k1_recover::Secp256k1RecoverError;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Secp256k1Error {
    #[error("The private key provided is invalid")]
    InvalidSecretKey,
    #[error("The public key provided is invalid")]
    InvalidPublicKey,
    #[error("Invalid Y coordinate")]
    InvalidYCoordinate,
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
}

impl From<Secp256k1RecoverError> for Secp256k1Error {
    fn from(_: Secp256k1RecoverError) -> Self {
        Secp256k1Error::InvalidPublicKey
    }
}
