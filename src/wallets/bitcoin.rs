use anyhow::Result;
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::SecretKey;
use bitcoin::util::key::PrivateKey;

use super::Wallet;
use crate::seed::Seed;

pub struct BitcoinWallet {
    private_key: PrivateKey,
}

impl BitcoinWallet {
    pub fn wif(&self) -> String {
        self.private_key.to_wif()
    }
}

impl Wallet for BitcoinWallet {
    fn from_seed(seed: &Seed) -> Result<BitcoinWallet> {
        Ok(BitcoinWallet {
            private_key: PrivateKey {
                compressed: true,
                network: Network::Bitcoin,
                key: SecretKey::from_slice(seed.to_bytes())?,
            },
        })
    }
}
