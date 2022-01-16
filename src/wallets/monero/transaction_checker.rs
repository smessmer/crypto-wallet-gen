use anyhow::Result;
use async_trait::async_trait;

use super::wallet::MoneroWallet;
use crate::wallets::TransactionChecker;

pub struct MoneroTransactionChecker {}

#[async_trait]
impl TransactionChecker<MoneroWallet> for MoneroTransactionChecker {
    async fn has_transactions(&self, wallet: &MoneroWallet) -> Result<bool> {
        todo!()
    }
}
