use anyhow::Result;
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::SecretKey;
use bitcoin::util::key::PrivateKey;

use crate::seed::Seed;

pub trait BitcoinWallet {
    fn wif(&self) -> String;
}

impl BitcoinWallet for PrivateKey {
    fn wif(&self) -> String {
        self.to_wif()
    }
}

pub fn seed_to_bitcoin_wallet(seed: impl Seed) -> Result<impl BitcoinWallet> {
    Ok(PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        key: SecretKey::from_slice(seed.as_bytes())?,
    })
}
