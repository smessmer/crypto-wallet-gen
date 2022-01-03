use anyhow::Result;
use async_trait::async_trait;
use failure::Fail;
use secp256k1_17::key::SecretKey;
use wagyu_ethereum::format::EthereumFormat;
use wagyu_ethereum::private_key::EthereumPrivateKey;
use wagyu_model::PrivateKey;

use super::TransactionChecker;
use super::Wallet;
use crate::bip32::{CoinType, HDPrivKey};

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

#[async_trait]
impl Wallet for EthereumWallet {
    type TransactionChecker = EthereumTransactionChecker;
    const COIN_TYPE: CoinType = CoinType::ETH;

    fn from_hd_key(private_key: &HDPrivKey) -> Result<Self> {
        let secp_key = SecretKey::from_slice(private_key.key_part().to_bytes())?;
        Ok(Self {
            private_key: EthereumPrivateKey::from_secp256k1_secret_key(secp_key),
        })
    }

    fn print_key(&self) -> Result<()> {
        println!(
            "Private Key: {}\nPublic Key: {}\nAddress: {}",
            self.private_key(),
            self.public_key(),
            self.address()?,
        );
        Ok(())
    }

    async fn new_transaction_checker() -> Result<EthereumTransactionChecker> {
        Ok(EthereumTransactionChecker {})
    }
}

pub struct EthereumTransactionChecker {}

#[async_trait]
impl TransactionChecker<EthereumWallet> for EthereumTransactionChecker {
    async fn has_transactions(&self, wallet: &EthereumWallet) -> Result<bool> {
        todo!()
    }
}
