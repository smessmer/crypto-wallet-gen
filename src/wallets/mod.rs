use anyhow::Result;
use async_trait::async_trait;

use crate::bip32::{CoinType, HDPrivKey};

pub mod bitcoin;
pub mod ethereum;
pub mod monero;

#[async_trait]
pub trait TransactionChecker<ConcreteWallet: Wallet> {
    async fn has_transactions(&self, wallet: &ConcreteWallet) -> Result<bool>;
}

#[async_trait]
pub trait Wallet: Sized {
    type TransactionChecker: TransactionChecker<Self>;
    const COIN_TYPE: CoinType;

    fn from_hd_key(private_key: &HDPrivKey) -> Result<Self>;

    async fn new_transaction_checker() -> Result<Self::TransactionChecker>;
    fn print_key(&self) -> Result<()>;
}
