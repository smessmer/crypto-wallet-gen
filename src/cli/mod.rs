use anyhow::{ensure, Result};
use clap::{crate_version, value_t, App, Arg, SubCommand};
use std::io::{self, Write};
use thiserror::Error;
use trompt::Trompt;

use crate::{Bip39Mnemonic, CoinType, Mnemonic, MnemonicFactory, ScryptMnemonic};

mod generate;
mod search;

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

pub async fn main() -> Result<()> {
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
            .about("Generate one or more wallet addresses")
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
        .subcommand(SubCommand::with_name("search")
            .about("Try different derivation paths and show all addresses that have transactions (i.e. have been used in the past)")
            .arg(
                Arg::with_name("stop-after-n-empty-accounts")
                    .long("stop-after-n-empty-accounts")
                    .value_name("NUM_ACCOUNTS")
                    .default_value("20")
                    .help("Stop searching for new accounts after n consecutive accounts didn't have any transactions")
            )
            .arg(
                Arg::with_name("stop-after-n-empty-change-indices")
                    .long("stop-after-n-empty-change-indices")
                    .value_name("NUM_CHANGE_INDICES")
                    .default_value("20")
                    .help("Stop searching for new change indices (within an account) after n consecutive change indices didn't have any transactions")
            )
            .arg(
                Arg::with_name("stop-after-n-empty-addresses")
                    .long("stop-after-n-empty-addresses")
                    .value_name("NUM_ADDRESSES")
                    .default_value("20")
                    .help("Stop searching for new addresses (within an account+change_index) after n consecutive addresses didn't have any transactions")
            )
        )
        .get_matches();

    let coin_type = value_t!(args, "coin", CoinType).unwrap_or_else(|e| e.exit());
    let scrypt = args.is_present("scrypt");
    let mnemonic = args.value_of("from-mnemonic");
    let mnemonic: Box<dyn Mnemonic> = if scrypt {
        Box::new(
            mnemonic
                .map(ScryptMnemonic::from_phrase)
                .unwrap_or_else(ScryptMnemonic::generate)?,
        )
    } else {
        Box::new(
            mnemonic
                .map(Bip39Mnemonic::from_phrase)
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
        generate::run(coin_type, &master_key, generate_args).await?;
    } else if let Some(search_args) = args.subcommand_matches("search") {
        search::run(coin_type, master_key, search_args).await?;
    } else {
        println!("Error: Please specify subcommand, e.g. 'generate' on the command line.");
    }

    Ok(())
}
