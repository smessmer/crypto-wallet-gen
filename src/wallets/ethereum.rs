use anyhow::Result;
use failure::Fail;
use secp256k1_17::key::SecretKey;
use wagyu_ethereum::format::EthereumFormat;
use wagyu_ethereum::private_key::EthereumPrivateKey;
use wagyu_model::PrivateKey;

use super::Wallet;
use crate::bip32::HDPrivKey;

pub struct EthereumWallet {
    private_key: EthereumPrivateKey,
}

impl EthereumWallet {
    pub fn private_key(&self) -> String {
        self.private_key.to_string()
    }

    pub fn public_key(&self) -> String {
        self.private_key.to_public_key().to_string()
    }

    pub fn address(&self) -> Result<String> {
        Ok(self
            .private_key
            .to_address(&EthereumFormat::Standard)
            .map_err(|err| err.compat())?
            .to_string())
    }
}

impl Wallet for EthereumWallet {
    fn from_hd_key(private_key: HDPrivKey) -> Result<Self> {
        let secp_key = SecretKey::from_slice(private_key.key_part().to_bytes())?;
        Ok(Self {
            private_key: EthereumPrivateKey::from_secp256k1_secret_key(secp_key),
        })
    }
}
