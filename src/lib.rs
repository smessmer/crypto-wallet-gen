use anyhow::Result;

mod bip32;
mod cli;
mod mnemonics;
mod random;
mod seed;
mod utils;
mod wallets;

pub use bip32::{Bip44DerivationPath, CoinType, HDPrivKey};
pub use mnemonics::{bip39::Bip39Mnemonic, scrypt::ScryptMnemonic, Mnemonic, MnemonicFactory};
pub use seed::Seed;
pub use wallets::{bitcoin::BitcoinWallet, ethereum::EthereumWallet, monero::MoneroWallet, Wallet};

pub async fn cli_main() -> Result<()> {
    cli::main().await
}
