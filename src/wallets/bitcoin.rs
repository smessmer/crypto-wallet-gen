use anyhow::Result;
use bitcoin::util::bip32::ExtendedPrivKey;

use super::Wallet;

pub struct BitcoinWallet {
    private_key: ExtendedPrivKey,
}

impl BitcoinWallet {
    pub fn private_key(&self) -> String {
        format!("{}", self.private_key)
    }
}

impl Wallet for BitcoinWallet {
    fn from_extended_key(private_key: ExtendedPrivKey) -> Result<Self> {
        Ok(Self { private_key })
    }
}
