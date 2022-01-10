use anyhow::{anyhow, Context, Result};
use clap::{value_t, ArgMatches};
use futures::future::{try_join, try_join4};
use std::iter::Peekable;

use crate::wallets::{
    bitcoin::BitcoinWallet, ethereum::EthereumWallet, monero::MoneroWallet, TransactionChecker,
    Wallet,
};
use crate::{Bip44DerivationPath, CoinType, HDPrivKey};

struct StopConditions {
    stop_after_n_empty_accounts: u32,
    stop_after_n_empty_change_indices: u32,
    stop_after_n_empty_addresses: u32,
}

pub async fn run(
    coin_type: CoinType,
    master_key: HDPrivKey,
    generate_args: &ArgMatches<'_>,
) -> Result<()> {
    let stop_conditions = StopConditions {
        stop_after_n_empty_accounts: value_t!(generate_args, "stop-after-n-empty-accounts", u32)
            .unwrap_or_else(|e| e.exit()),
        stop_after_n_empty_change_indices: value_t!(
            generate_args,
            "stop-after-n-empty-change-indices",
            u32
        )
        .unwrap_or_else(|e| e.exit()),
        stop_after_n_empty_addresses: value_t!(generate_args, "stop-after-n-empty-addresses", u32)
            .unwrap_or_else(|e| e.exit()),
    };
    match coin_type {
        CoinType::BTC => Searcher::<BitcoinWallet>::run(master_key, stop_conditions).await,
        CoinType::ETH => Searcher::<EthereumWallet>::run(master_key, stop_conditions).await,
        CoinType::XMR => Searcher::<MoneroWallet>::run(master_key, stop_conditions).await,
    }
}

struct Searcher<ConcreteWallet: Wallet> {
    master_key: HDPrivKey,
    transaction_checker: ConcreteWallet::TransactionChecker,
    stop_conditions: StopConditions,
}

impl<ConcreteWallet: Wallet> Searcher<ConcreteWallet> {
    pub async fn run(master_key: HDPrivKey, stop_conditions: StopConditions) -> Result<()> {
        let transaction_checker = ConcreteWallet::new_transaction_checker().await?;
        let searcher = Self {
            master_key,
            transaction_checker,
            stop_conditions,
        };
        let found_addresses = searcher._search_accounts().await?;
        println!("Found the following addresses with transactions:");
        for (derivation_path, wallet) in found_addresses {
            println!(
                "--------------------------------------------------------------------------------------\nBIP44 Derivation Path: {}\n",
                derivation_path.map(|p| p.to_string()).unwrap_or_else(|| String::from("none")),
            );
            wallet.print_key()?;
        }
        Ok(())
    }

    async fn _search_accounts(
        &self,
    ) -> Result<impl Iterator<Item = (Option<Bip44DerivationPath>, ConcreteWallet)> + '_> {
        let wallet_from_root_path = async move {
            self._wallet_if_has_transactions(None)
                .await
                .map(|a| a.into_iter())
        };
        let wallet_from_intermediate_path_1 = async move {
            self._wallet_if_has_transactions(Some(Bip44DerivationPath {
                coin_type: None,
                account: None,
                change: None,
                address_index: None,
            }))
            .await
            .map(|a| a.into_iter())
        };
        let wallet_from_intermediate_path_2 = async move {
            self._wallet_if_has_transactions(Some(Bip44DerivationPath {
                coin_type: Some(ConcreteWallet::COIN_TYPE),
                account: None,
                change: None,
                address_index: None,
            }))
            .await
            .map(|a| a.into_iter())
        };
        let wallets_from_derived_paths = async move {
            Ok(crate::utils::search::search(
                self.stop_conditions.stop_after_n_empty_accounts,
                move |account_index| {
                    Box::pin(async move {
                        Ok(none_if_empty(
                            self._search_changes(account_index).await?.peekable(),
                        ))
                    })
                },
            )
            .await?
            .flatten())
        };
        let (
            wallet_from_root_path,
            wallet_from_intermediate_path_1,
            wallet_from_intermediate_path_2,
            wallets_from_derived_paths,
        ) = try_join4(
            wallet_from_root_path,
            wallet_from_intermediate_path_1,
            wallet_from_intermediate_path_2,
            wallets_from_derived_paths,
        )
        .await?;
        Ok([
            wallet_from_root_path,
            wallet_from_intermediate_path_1,
            wallet_from_intermediate_path_2,
        ]
        .into_iter()
        .flatten()
        .chain(wallets_from_derived_paths))
    }

    async fn _search_changes(
        &self,
        account_index: u32,
    ) -> Result<impl Iterator<Item = (Option<Bip44DerivationPath>, ConcreteWallet)> + '_> {
        let wallet_from_intermediate_path = async move {
            self._wallet_if_has_transactions(Some(Bip44DerivationPath {
                coin_type: Some(ConcreteWallet::COIN_TYPE),
                account: Some(account_index),
                change: None,
                address_index: None,
            }))
            .await
            .map(|a| a.into_iter())
        };
        let wallets_from_derived_paths = async move {
            Ok(crate::utils::search::search(
                self.stop_conditions.stop_after_n_empty_change_indices,
                move |change_index| {
                    Box::pin(async move {
                        Ok(none_if_empty(
                            self._search_addresses(account_index, change_index)
                                .await?
                                .peekable(),
                        ))
                    })
                },
            )
            .await?
            .flatten())
        };
        let (wallet_from_intermediate_path, wallets_from_derived_paths) =
            try_join(wallet_from_intermediate_path, wallets_from_derived_paths).await?;
        Ok(wallet_from_intermediate_path.chain(wallets_from_derived_paths))
    }

    async fn _search_addresses(
        &self,
        account_index: u32,
        change_index: u32,
    ) -> Result<impl Iterator<Item = (Option<Bip44DerivationPath>, ConcreteWallet)> + '_> {
        let wallet_from_intermediate_path = async move {
            Ok(self
                ._wallet_if_has_transactions(Some(Bip44DerivationPath {
                    coin_type: Some(ConcreteWallet::COIN_TYPE),
                    account: Some(account_index),
                    change: Some(change_index),
                    address_index: None,
                }))
                .await?
                .into_iter())
        };
        let wallets_from_derived_paths = crate::utils::search::search(
            self.stop_conditions.stop_after_n_empty_addresses,
            move |address_index| {
                Box::pin(self._wallet_if_has_transactions(Some(Bip44DerivationPath {
                    coin_type: Some(ConcreteWallet::COIN_TYPE),
                    account: Some(account_index),
                    change: Some(change_index),
                    address_index: Some(address_index),
                })))
            },
        );
        let (wallet_from_intermediate_path, wallets_from_derived_paths) =
            try_join(wallet_from_intermediate_path, wallets_from_derived_paths).await?;
        Ok(wallet_from_intermediate_path.chain(wallets_from_derived_paths))
    }

    async fn _wallet_if_has_transactions(
        &self,
        derivation_path: Option<Bip44DerivationPath>,
    ) -> Result<Option<(Option<Bip44DerivationPath>, ConcreteWallet)>> {
        // TODO logging
        // let derivation_path_str = derivation_path
        //         .as_ref()
        //         .map(|t| t.to_string())
        //         .unwrap_or_else(|| String::from("m"));
        // log::debug!(
        //     "Checking {}",
        //     derivation_path_str,
        //     has_transactions,
        // );
        let derived = if let Some(derivation_path) = &derivation_path {
            self.master_key
                .derive(&derivation_path)
                .with_context(|| anyhow!("Error deriving master key to {}", derivation_path))?
        } else {
            self.master_key.clone()
        };
        let wallet =
            ConcreteWallet::from_hd_key(&derived).context("Error creating wallet from hd key")?;
        let has_transactions = self
            .transaction_checker
            .has_transactions(&wallet)
            .await
            .context("Error checking whether wallet has transactions")?;
        // TODO logging
        // log::info!(
        //     "Checked {}: {}",
        //     derivation_path_str,
        //     if has_transactions {"has transactions"} else {"has no transactions"},
        // );
        if has_transactions {
            Ok(Some((derivation_path, wallet)))
        } else {
            Ok(None)
        }
    }
}

// Converts an empty iterator to None and a non-empty iterator to Some
fn none_if_empty<T, I: Iterator<Item = T>>(mut iter: Peekable<I>) -> Option<Peekable<I>> {
    if iter.peek().is_none() {
        None
    } else {
        Some(iter)
    }
}
