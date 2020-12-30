use anyhow::Result;
use bitcoin::network::constants::Network;
use bitcoin::util::bip32::ExtendedPrivKey;
use clap::arg_enum;
use secp256k1::Secp256k1;
use std::convert::TryFrom;
use std::convert::TryInto;

use crate::seed::Seed;

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

impl TryFrom<Bip44DerivationPath> for bitcoin::util::bip32::DerivationPath {
    type Error = anyhow::Error;

    fn try_from(path: Bip44DerivationPath) -> Result<bitcoin::util::bip32::DerivationPath> {
        use bitcoin::util::bip32::ChildNumber;
        let mut path_vec = vec![
            ChildNumber::from_hardened_idx(44).expect("44 is a valid index"),
            ChildNumber::from_hardened_idx(path.coin_type.bip44_value())?,
            ChildNumber::from_hardened_idx(path.account)?,
        ];
        if let Some(change) = path.change {
            path_vec.push(ChildNumber::from_normal_idx(change)?);
        } else {
            assert!(
                path.address_index.is_none(),
                "address_index can only be set when change is set"
            );
        }
        if let Some(address_index) = path.address_index {
            path_vec.push(ChildNumber::from_normal_idx(address_index)?);
        }
        Ok(path_vec.into())
    }
}

pub struct HDPrivKey {
    ext_key: ExtendedPrivKey,
}

impl HDPrivKey {
    pub fn new(master_seed: Seed) -> Result<Self> {
        Ok(Self {
            ext_key: ExtendedPrivKey::new_master(Network::Bitcoin, master_seed.to_bytes())?,
        })
    }

    pub fn derive(&self, path: Bip44DerivationPath) -> Result<HDPrivKey> {
        let secp256k1 = Secp256k1::new();
        let path: bitcoin::util::bip32::DerivationPath = path.try_into()?;
        Ok(HDPrivKey {
            ext_key: self.ext_key.derive_priv(&secp256k1, &path)?,
        })
    }

    pub fn key_part(&self) -> Seed {
        Seed::from_bytes(self.ext_key.private_key.to_bytes())
    }

    pub fn to_base58(&self) -> String {
        format!("{}", self.ext_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account0() {
        // Generated with https://iancoleman.io/bip39/
        let master_seed = hex::decode("04c3fca05109eb0d188971e66ba949a4a4547b6c0eceddcb3e796e6ddb7d489826901932dbab5d6aa71421de1d119b4d472a92702e2642b2d9259d4766d84284").unwrap();
        let child_key = HDPrivKey::new(Seed::from_bytes(master_seed))
            .unwrap()
            .derive(Bip44DerivationPath {
                coin_type: CoinType::BTC,
                account: 0,
                change: Some(0),
                address_index: None,
            })
            .unwrap();
        assert_eq!(
            "xprvA1gz733iMcZ7hmAwuWdzw6suwn3ScGtpjGH7qzdFTKqtMvyRyBZ92n3fpvLahFnqXpA13NwPktkkCumeaRQpRg7iNkcvUoBu4T1eK4fhNDv",
            child_key.to_base58(),
        );
    }

    #[test]
    fn test_account1() {
        // Generated with https://iancoleman.io/bip39/
        let master_seed = hex::decode("04c3fca05109eb0d188971e66ba949a4a4547b6c0eceddcb3e796e6ddb7d489826901932dbab5d6aa71421de1d119b4d472a92702e2642b2d9259d4766d84284").unwrap();
        let child_key = HDPrivKey::new(Seed::from_bytes(master_seed))
            .unwrap()
            .derive(Bip44DerivationPath {
                coin_type: CoinType::BTC,
                account: 1,
                change: Some(0),
                address_index: None,
            })
            .unwrap();
        assert_eq!(
            "xprvA2M4iy8qw2abD2MqssXJvtVU1p9AHHFPiqcSZzj28Gt1ZGwJ4oXLGQUK1R7JYQgtHA54t3yiKtSGgSVHwvxA1YJV7R7pbUefWa6u1E61rbS",
            child_key.to_base58(),
        );
    }
}
