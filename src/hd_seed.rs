use anyhow::Result;
use clap::arg_enum;
use thiserror::Error;
use tiny_hderive::bip32::ExtendedPrivKey;

use crate::seed::Seed;

// TODO Secp256k1Error is only needed because libsecp256k1::Error in version 0.2 doesn't implement std::error::Error yet. It does in a newer version, but tiny_hderive locks us to 0.2.
#[derive(Error, Debug)]
pub enum Secp256k1Error {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid secret key")]
    InvalidSecretKey,
    #[error("Invalid recovery id")]
    InvalidRecoveryId,
    #[error("Invalid message")]
    InvalidMessage,
    #[error("Invalid input length")]
    InvalidInputLength,
    #[error("Tweak out of range")]
    TweakOutOfRange,
}

impl From<secp256k1::Error> for Secp256k1Error {
    fn from(err: secp256k1::Error) -> Secp256k1Error {
        match err {
            secp256k1::Error::InvalidSignature => Secp256k1Error::InvalidSignature,
            secp256k1::Error::InvalidPublicKey => Secp256k1Error::InvalidPublicKey,
            secp256k1::Error::InvalidSecretKey => Secp256k1Error::InvalidSecretKey,
            secp256k1::Error::InvalidRecoveryId => Secp256k1Error::InvalidRecoveryId,
            secp256k1::Error::InvalidMessage => Secp256k1Error::InvalidMessage,
            secp256k1::Error::InvalidInputLength => Secp256k1Error::InvalidInputLength,
            secp256k1::Error::TweakOutOfRange => Secp256k1Error::TweakOutOfRange,
        }
    }
}

// TODO DeriveError is only needed because tiny_hderive::Error doesn't implement std::error::Error. We should probably upstream a pull request.
#[derive(Error, Debug)]
pub enum DeriveError {
    #[error(transparent)]
    Secp256k1(#[from] Secp256k1Error),
    #[error("Invalid derivation path")]
    InvalidDerivationPath,
    #[error("Invalid child number")]
    InvalidChildNumber,
    #[error("Invalid extended priv key")]
    InvalidExtendedPrivKey,
}

impl From<tiny_hderive::Error> for DeriveError {
    fn from(err: tiny_hderive::Error) -> DeriveError {
        match err {
            tiny_hderive::Error::Secp256k1(err) => {
                DeriveError::Secp256k1(Secp256k1Error::from(err))
            }
            tiny_hderive::Error::InvalidDerivationPath => DeriveError::InvalidDerivationPath,
            tiny_hderive::Error::InvalidChildNumber => DeriveError::InvalidChildNumber,
            tiny_hderive::Error::InvalidExtendedPrivKey => DeriveError::InvalidExtendedPrivKey,
        }
    }
}

arg_enum! {
    #[derive(Debug, Clone, Copy)]
    pub enum CoinType {
        // List: https://github.com/libbitcoin/libbitcoin-system/wiki/Altcoin-Version-Mappings#10-monero-xmr-bip-3944-technology-examples
        BTC,
        XMR,
    }
}

impl CoinType {
    fn bip44_value(self) -> u32 {
        match self {
            Self::BTC => 0,
            Self::XMR => 128,
        }
    }
}

#[derive(Debug)]
pub struct Bip44DerivationPath {
    pub coin_type: CoinType,
    pub account: u32,
    pub change: Option<u32>,
    pub address_index: Option<u32>,
}

impl tiny_hderive::bip44::IntoDerivationPath for Bip44DerivationPath {
    fn into(self) -> Result<tiny_hderive::bip44::DerivationPath, tiny_hderive::Error> {
        // TODO Faster implementation if not going through intermediate string representation
        let mut path_str = format!("m/44'/{}'/{}'", self.coin_type.bip44_value(), self.account);
        if let Some(change) = self.change {
            path_str += &format!("/{}", change);
        } else {
            assert!(
                self.address_index.is_none(),
                "address_index can only be set when change is set"
            );
        }
        if let Some(address_index) = self.address_index {
            path_str += &format!("/{}", address_index);
        }
        use std::str::FromStr;
        tiny_hderive::bip44::DerivationPath::from_str(&path_str)
    }
}

pub struct HDSeed {
    master_seed: Seed,
}

impl HDSeed {
    pub fn new(master_seed: Seed) -> Self {
        Self { master_seed }
    }

    pub fn derive(&self, path: Bip44DerivationPath) -> Result<Seed> {
        let ext = ExtendedPrivKey::derive(self.master_seed.to_bytes(), path)
            .map_err(DeriveError::from)?;
        Ok(Seed::from_bytes(ext.secret().to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account0() {
        // Generated with https://iancoleman.io/bip39/ and using "bx hd-to-ec xprvA1gz733iMcZ7hmAwuWdzw6suwn3ScGtpjGH7qzdFTKqtMvyRyBZ92n3fpvLahFnqXpA13NwPktkkCumeaRQpRg7iNkcvUoBu4T1eK4fhNDv"
        let master_seed = hex::decode("04c3fca05109eb0d188971e66ba949a4a4547b6c0eceddcb3e796e6ddb7d489826901932dbab5d6aa71421de1d119b4d472a92702e2642b2d9259d4766d84284").unwrap();
        let child_seed = HDSeed::new(Seed::from_bytes(master_seed))
            .derive(Bip44DerivationPath {
                coin_type: CoinType::BTC,
                account: 0,
                change: Some(0),
                address_index: None,
            })
            .unwrap();
        assert_eq!(
            "d2b621b864d8aa9ff26dc32346868ea13e63ed0185dee5954d5615fc2381c4a3",
            hex::encode(child_seed.to_bytes())
        );
    }

    #[test]
    fn test_account1() {
        // Generated with https://iancoleman.io/bip39/ and using "bx hd-to-ec xprvA2M4iy8qw2abD2MqssXJvtVU1p9AHHFPiqcSZzj28Gt1ZGwJ4oXLGQUK1R7JYQgtHA54t3yiKtSGgSVHwvxA1YJV7R7pbUefWa6u1E61rbS"
        let master_seed = hex::decode("04c3fca05109eb0d188971e66ba949a4a4547b6c0eceddcb3e796e6ddb7d489826901932dbab5d6aa71421de1d119b4d472a92702e2642b2d9259d4766d84284").unwrap();
        let child_seed = HDSeed::new(Seed::from_bytes(master_seed))
            .derive(Bip44DerivationPath {
                coin_type: CoinType::BTC,
                account: 1,
                change: Some(0),
                address_index: None,
            })
            .unwrap();
        assert_eq!(
            "f258d2fb41fe5af9295d8fffd5f4575a54314772fd812905ebb1e5f3554b7fc8",
            hex::encode(child_seed.to_bytes())
        );
    }

    // TODO Tests for XMR
}
