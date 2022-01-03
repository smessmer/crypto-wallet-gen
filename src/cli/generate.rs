use anyhow::{Context, Result};
use clap::ArgMatches;

use crate::{
    Bip44DerivationPath, BitcoinWallet, CoinType, EthereumWallet, HDPrivKey, MoneroWallet, Wallet,
};

pub fn run(coin_type: CoinType, master_key: &HDPrivKey, generate_args: &ArgMatches) -> Result<()> {
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

    for account_index in account_indices {
        if let Some(address_indices) = &address_indices {
            let change_indices = change_indices
                .as_ref()
                .expect("When address-index is defined, change-index must be defined as well");
            for change_index in change_indices {
                for address_index in address_indices {
                    let derivation_path = Bip44DerivationPath {
                        coin_type,
                        account: account_index,
                        change: Some(*change_index),
                        address_index: Some(*address_index),
                    };
                    print_key(coin_type, master_key, &derivation_path)?;
                }
            }
        } else {
            let derivation_path = Bip44DerivationPath {
                coin_type,
                account: account_index,
                change: None,
                address_index: None,
            };
            print_key(coin_type, master_key, &derivation_path)?;
        }
    }

    Ok(())
}

fn print_key(
    coin_type: CoinType,
    master_key: &HDPrivKey,
    derivation_path: &Bip44DerivationPath,
) -> Result<()> {
    println!(
        "--------------------------------------------------------------------------------------\nBIP44 Derivation Path: {}",
        derivation_path,
    );
    let derived = derive_key(master_key, derivation_path)?;
    match coin_type {
        CoinType::XMR => {
            let wallet = MoneroWallet::from_hd_key(&derived)?;
            wallet.print_key()?;
        }
        CoinType::BTC => {
            let wallet = BitcoinWallet::from_hd_key(&derived)?;
            wallet.print_key()?;
        }
        CoinType::ETH => {
            let wallet = EthereumWallet::from_hd_key(&derived)?;
            wallet.print_key()?;
        }
    }
    Ok(())
}

fn derive_key(master_key: &HDPrivKey, path: &Bip44DerivationPath) -> Result<HDPrivKey> {
    master_key.derive(path)
}
