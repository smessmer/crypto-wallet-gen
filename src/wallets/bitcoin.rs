use anyhow::Result;
use async_trait::async_trait;

use super::TransactionChecker;
use super::Wallet;
use crate::bip32::{CoinType, HDPrivKey};

pub struct BitcoinWallet {
    private_key: HDPrivKey,
}

impl BitcoinWallet {
    pub fn private_key(&self) -> String {
        self.private_key.to_base58()
    }
}

#[async_trait]
impl Wallet for BitcoinWallet {
    type TransactionChecker = BitcoinTransactionChecker;
    const COIN_TYPE: CoinType = CoinType::BTC;

    fn from_hd_key(private_key: &HDPrivKey) -> Result<Self> {
        Ok(Self {
            private_key: private_key.clone(),
        })
    }

    fn print_key(&self) -> Result<()> {
        println!("Private Key: {}", self.private_key());
        Ok(())
    }

    async fn new_transaction_checker() -> Result<BitcoinTransactionChecker> {
        Ok(BitcoinTransactionChecker {})
    }
}

pub struct BitcoinTransactionChecker {}

#[async_trait]
impl TransactionChecker<BitcoinWallet> for BitcoinTransactionChecker {
    async fn has_transactions(&self, wallet: &BitcoinWallet) -> Result<bool> {
        todo!()
    }
}
