use solana_nostd_secp256k1_recover::Secp256k1RecoverError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Secp256k1Error {
    InvalidSecretKey,
    InvalidPublicKey,
    InvalidYCoordinate,
    ArithmeticOverflow,
}

impl From<Secp256k1RecoverError> for Secp256k1Error {
    fn from(_: Secp256k1RecoverError) -> Self {
        Secp256k1Error::InvalidPublicKey
    }
}
