use anyhow::{ensure, Context, Result};
use clap::{crate_version, value_t, App, Arg};
use thiserror::Error;
use trompt::Trompt;

use crypto_wallet_gen::{
    Bip39Mnemonic, Bip44DerivationPath, BitcoinWallet, CoinType, Mnemonic, MoneroWallet, Wallet,
};

// TODO This is only needed because trompt::Error doesn't implement std::error::TromptError. We should upstream a fix instead.
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
            Arg::with_name("account-index")
                .short("a")
                .long("account-index")
                .default_value("0")
                .value_name("ACCOUNT INDEX")
                .help("The account index used for BIP44 key derivation"),
        )
        .get_matches();

    let coin_type = value_t!(args, "coin", CoinType).unwrap_or_else(|e| e.exit());
    let mnemonic = args
        .value_of("from-mnemonic")
        .map(Bip39Mnemonic::from_phrase)
        .unwrap_or_else(|| Ok(Bip39Mnemonic::generate()))?;
    let account_index: u32 = args
        .value_of("account-index")
        .expect("Can't fail because we specify a default value")
        .parse()
        .context("Couldn't parse account-index argument")?;
    let password1 = Trompt::stdout()
        .silent()
        .prompt("Password: ")
        .map_err(TromptError::from)?;
    let password2 = Trompt::stdout()
        .silent()
        .prompt("Repeat Password: ")
        .map_err(TromptError::from)?;
    ensure!(password1 == password2, "Passwords don't match");

    let master_seed = mnemonic.to_seed(&password1);
    let derived = master_seed.derive(Bip44DerivationPath {
        coin_type,
        account: account_index,
        change: Some(0),
        address_index: Some(0),
    })?;
    match coin_type {
        CoinType::XMR => {
            let wallet = MoneroWallet::from_seed(&derived)?;

            println!(
                "Mnemonic: {}\nPassword: [omitted]\nAddress: {}\nPrivate View Key: {}\nPrivate Spend Key: {}",
                mnemonic.phrase(),
                wallet.address(),
                wallet.private_view_key()?,
                wallet.private_spend_key(),
            );
        }
        CoinType::BTC => {
            let wallet = BitcoinWallet::from_seed(&derived)?;

            println!(
                "Mnemonic: {}\nPassword: [omitted]\nWIF: {}",
                mnemonic.phrase(),
                wallet.wif(),
            );
        }
    }

    Ok(())
}
