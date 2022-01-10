use anyhow::{Context, Result};
use clap::ArgMatches;
use futures::future::{self, LocalBoxFuture};
use std::future::Future;
use std::pin::Pin;

use crate::{
    Bip44DerivationPath, BitcoinWallet, CoinType, EthereumWallet, HDPrivKey, MoneroWallet, Wallet,
};

pub async fn run(
    coin_type: CoinType,
    master_key: &HDPrivKey,
    generate_args: &ArgMatches<'_>,
) -> Result<()> {
    let account_indices: Option<Vec<u32>> = generate_args
        .values_of("account-index")
        .map_or(Ok(None), |v| {
            v.map(|v| v.parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()
                .map(Some)
        })
        .context("Couldn't parse account-index argument")?;
    let change_indices: Option<Vec<u32>> = generate_args
        .values_of("change-index")
        .map_or(Ok(None), |v| {
            v.map(|v| v.parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()
                .map(Some)
        })
        .context("Couldn't parse change-index argument")?;
    let address_indices: Option<Vec<u32>> = generate_args
        .values_of("address-index")
        .map_or(Ok(None), |v| {
            v.map(|v| v.parse::<u32>())
                .collect::<Result<Vec<u32>, _>>()
                .map(Some)
        })
        .context("Couldn't parse address-index argument")?;
    if address_indices.is_some() && change_indices.is_none() {
        panic!("--address-index can only be specified if --change-index is also specified.");
    }

    let account_indices = account_indices.unwrap_or_else(|| vec![0, 1, 2]);

    let keys_to_print = account_indices.into_iter().flat_map(
        |account_index| -> Box<
            dyn Iterator<
                Item = Pin<Box<dyn Future<Output = Result<(Bip44DerivationPath, HDPrivKey)>>>>,
            >,
        > {
            if let Some(address_indices) = &address_indices {
                let change_indices = change_indices
                    .as_ref()
                    .expect("When address-index is defined, change-index must be defined as well");
                Box::new(generate_keys_for_account(
                    master_key,
                    coin_type,
                    account_index,
                    &change_indices,
                    &address_indices,
                ))
            } else {
                Box::new(generate_root_key_for_account(
                    master_key,
                    coin_type,
                    account_index,
                ))
            }
        },
    );
    let keys_to_print = future::try_join_all(keys_to_print).await?;
    for (derivation_path, derived_key) in keys_to_print {
        print_key(coin_type, &derivation_path, &derived_key)?;
    }

    Ok(())
}

fn generate_keys_for_account<'a>(
    master_key: &'a HDPrivKey,
    coin_type: CoinType,
    account_index: u32,
    change_indices: &'a [u32],
    address_indices: &'a [u32],
) -> impl Iterator<Item = LocalBoxFuture<'a, Result<(Bip44DerivationPath, HDPrivKey)>>> {
    change_indices.into_iter().flat_map(move |change_index| {
        address_indices.into_iter().map(move |address_index| {
            let derivation_path = Bip44DerivationPath {
                coin_type: Some(coin_type),
                account: Some(account_index),
                change: Some(*change_index),
                address_index: Some(*address_index),
            };
            let r: LocalBoxFuture<_> = Box::pin(async move {
                let derived_key = derive_key(master_key, &derivation_path).await?;
                Ok((derivation_path, derived_key))
            });
            r
        })
    })
}

fn generate_root_key_for_account(
    master_key: &HDPrivKey,
    coin_type: CoinType,
    account_index: u32,
) -> impl Iterator<Item = LocalBoxFuture<Result<(Bip44DerivationPath, HDPrivKey)>>> {
    let derivation_path = Bip44DerivationPath {
        coin_type: Some(coin_type),
        account: Some(account_index),
        change: None,
        address_index: None,
    };
    let r: LocalBoxFuture<_> = Box::pin(async move {
        let derived_key = derive_key(master_key, &derivation_path).await?;
        Ok((derivation_path, derived_key))
    });
    std::iter::once(r)
}

fn print_key(
    coin_type: CoinType,
    derivation_path: &Bip44DerivationPath,
    derived_key: &HDPrivKey,
) -> Result<()> {
    println!(
        "--------------------------------------------------------------------------------------\nBIP44 Derivation Path: {}",
        derivation_path,
    );
    match coin_type {
        CoinType::XMR => {
            let wallet = MoneroWallet::from_hd_key(&derived_key)?;
            wallet.print_key()?;
        }
        CoinType::BTC => {
            let wallet = BitcoinWallet::from_hd_key(&derived_key)?;
            wallet.print_key()?;
        }
        CoinType::ETH => {
            let wallet = EthereumWallet::from_hd_key(&derived_key)?;
            wallet.print_key()?;
        }
    }
    Ok(())
}

async fn derive_key(master_key: &HDPrivKey, path: &Bip44DerivationPath) -> Result<HDPrivKey> {
    master_key.derive_async(path).await
}
