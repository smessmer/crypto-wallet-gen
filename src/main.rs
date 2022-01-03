use anyhow::{ensure, Context, Result};
use clap::{crate_version, value_t, App, Arg, ArgMatches, SubCommand};
use std::io::{self, Write};
use thiserror::Error;
use trompt::Trompt;

use crypto_wallet_gen::{
    Bip39Mnemonic, Bip44DerivationPath, BitcoinWallet, CoinType, EthereumWallet, HDPrivKey,
    Mnemonic, MnemonicFactory, MoneroWallet, ScryptMnemonic, Wallet,
};

// TODO This is only needed because trompt::Error doesn't implement std::error::TromptError. https://gitlab.com/runarberg/trompt/-/issues/4
#[derive(Debug, Error)]
pub enum TromptValidationError {
    #[error("absent")]
    Absent,
    #[error("too long")]
    TooLong,
    #[error("too short")]
    TooShort,
    #[error("unexpected input: {0}")]
    UnexpectedInput(String),
    #[error("other: {0}")]
    Other(String),
}
impl From<trompt::ValidationError> for TromptValidationError {
    fn from(err: trompt::ValidationError) -> TromptValidationError {
        match err {
            trompt::ValidationError::Absent => TromptValidationError::Absent,
            trompt::ValidationError::TooLong => TromptValidationError::TooLong,
            trompt::ValidationError::TooShort => TromptValidationError::TooShort,
            trompt::ValidationError::UnexpectedInput(input) => {
                TromptValidationError::UnexpectedInput(input)
            }
            trompt::ValidationError::Other(reason) => TromptValidationError::Other(reason),
        }
    }
}
#[derive(Debug, Error)]
pub enum TromptError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Validation(#[from] TromptValidationError),
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
}
impl From<trompt::Error> for TromptError {
    fn from(err: trompt::Error) -> TromptError {
        match err {
            trompt::Error::Io(err) => TromptError::Io(err),
            trompt::Error::Validation(err) => {
                TromptError::Validation(TromptValidationError::from(err))
            }
            trompt::Error::FromUtf8(err) => TromptError::FromUtf8(err),
        }
    }
}

fn main() -> Result<()> {
    let args = App::new("Crypto Wallet Generator")
        .version(crate_version!())
        .author("Sebastian Messmer <mail@smessmer.de>")
        .about("Generates crypto currency wallets from mnemonic seeds")
        .arg(
            Arg::with_name("coin")
                .short("c")
                .long("coin")
                .possible_values(&CoinType::variants())
                .value_name("COIN")
                .case_insensitive(true)
                .required(true)
                .help("The crypto coin to generate a wallet for"),
        )
        .arg(
            Arg::with_name("from-mnemonic")
                .short("m")
                .long("from-mnemonic")
                .value_name("MNEMONIC SEED PHRASE")
                .case_insensitive(true)
                .help("The mnemonic seed phrase to use to generate the wallet"),
        )
        .arg(
            Arg::with_name("scrypt")
            .short("s")
            .long("scrypt")
            .help("Use scrypt instead of PBKDF2 in the BIP39 derivation. This makes keys harder to brute force, but it deviates from the BIP39 standard.")
        )
        .subcommand(SubCommand::with_name("generate")
            .arg(
                Arg::with_name("account-index")
                    .short("a")
                    .long("account-index")
                    .multiple(true)
                    .value_name("INDEX")
                    .help("The account index used for BIP44 key derivation"),
            )
            .arg(
                Arg::with_name("change-index")
                    .long("change-index")
                    .multiple(true)
                    .value_name("INDEX")
                    .help("The change part of the BIP44 derivation path. If this parameter is not specified, we'll use a BIP44 path ending before the change part.")
            )
            .arg(
                Arg::with_name("address-index")
                    .long("address-index")
                    .multiple(true)
                    .value_name("INDEX")
                    .help("The address index part of the BIP44 derivation path. If this parameter is not specified, we'll use a BIP44 path ending before the address index part.")
            )
        )
        .get_matches();

    let coin_type = value_t!(args, "coin", CoinType).unwrap_or_else(|e| e.exit());
    let scrypt = args.is_present("scrypt");
    let mnemonic = args.value_of("from-mnemonic");
    let mnemonic: Box<dyn Mnemonic> = if scrypt {
        Box::new(
            mnemonic
                .map(|m| ScryptMnemonic::from_phrase(m))
                .unwrap_or_else(ScryptMnemonic::generate)?,
        )
    } else {
        Box::new(
            mnemonic
                .map(|m| Bip39Mnemonic::from_phrase(m))
                .unwrap_or_else(Bip39Mnemonic::generate)?,
        )
    };
    let password1 = Trompt::stdout()
        .silent()
        .prompt("Password: ")
        .map_err(TromptError::from)?;
    let password2 = Trompt::stdout()
        .silent()
        .prompt("Repeat Password: ")
        .map_err(TromptError::from)?;
    ensure!(password1 == password2, "Passwords don't match");

    if scrypt {
        print!("Generating keys with scrypt. This can take a while...");
        io::stdout().lock().flush().expect("Flushing stdout failed");
    }
    let master_key = mnemonic.to_private_key(&password1)?;
    if scrypt {
        println!("done");
    }
    println!(
        "Mnemonic: {}\nPassword: [omitted from output]",
        mnemonic.phrase()
    );

    if let Some(generate_args) = args.subcommand_matches("generate") {
        handle_generate(coin_type, &master_key, generate_args)?;
    } else {
        println!("Error: Please specify subcommand, e.g. 'generate' on the command line.");
    }

    Ok(())
}

fn handle_generate(
    coin_type: CoinType,
    master_key: &HDPrivKey,
    generate_args: &ArgMatches,
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
                    print_key(coin_type, &master_key, &derivation_path)?;
                }
            }
        } else {
            let derivation_path = Bip44DerivationPath {
                coin_type,
                account: account_index,
                change: None,
                address_index: None,
            };
            print_key(coin_type, &master_key, &derivation_path)?;
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
            let wallet = MoneroWallet::from_hd_key(derived)?;

            println!(
                "Address: {}\nPrivate View Key: {}\nPrivate Spend Key: {}",
                wallet.address()?,
                wallet.private_view_key(),
                wallet.private_spend_key(),
            );
        }
        CoinType::BTC => {
            let wallet = BitcoinWallet::from_hd_key(derived)?;

            println!("Private Key: {}", wallet.private_key(),);
        }
        CoinType::ETH => {
            let wallet = EthereumWallet::from_hd_key(derived)?;

            println!(
                "Private Key: {}\nPublic Key: {}\nAddress: {}",
                wallet.private_key(),
                wallet.public_key(),
                wallet.address()?,
            );
        }
    }
    Ok(())
}

fn derive_key(master_key: &HDPrivKey, path: &Bip44DerivationPath) -> Result<HDPrivKey> {
    master_key.derive(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electrum_derivation_matches_bip44() {
        // Test that when importing a derived key into electrum, electrum generates the correct BIP44 keys.
        // To test this, we generated a mnemonic at https://iancoleman.io/bip39/
        let mnemonic = "giggle load civil velvet legend drink letter symbol vivid tube parent plug accuse fault choose ahead bomb make novel potato enrich honey cable exchange";
        // We then use our tool to generate the private key
        let master_seed = Bip39Mnemonic::from_phrase(mnemonic)
            .unwrap()
            .to_private_key("")
            .unwrap();
        assert_eq!(
            "xprv9zEiTz4LvP1k9brLSck5yX41EzVi3xbC2ZkPhWdyTqvJu3ovQCD6R8Z8RUoTwKkwpdqMne95zSrk9duV2SYhmmRkxvZAMsdqNHThKP8STbi",
            derive_key(&master_seed, &Bip44DerivationPath {
                coin_type: CoinType::BTC, account: 0, change: None, address_index: None}).unwrap().to_base58(),
        );
        // and loaded that key into electrum, checking that electrum generates the BIP44 addresses
        // listed on https://iancoleman.io/bip39/
        // So this test case is basically a test ensuring that we keep generating the same private key for which we already checked
        // what electrum generates from it and don't start differring from it.
    }
}
