use anyhow::{ensure, Context, Result};
use clap::{crate_version, value_t, App, Arg};
use std::io::{self, Write};
use thiserror::Error;
use trompt::Trompt;

use crypto_wallet_gen::{
    Bip39Mnemonic, Bip44DerivationPath, BitcoinWallet, CoinType, HDPrivKey, Mnemonic,
    MnemonicFactory, MoneroWallet, ScryptMnemonic, Wallet,
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
        .arg(
            Arg::with_name("scrypt")
            .short("s")
            .long("scrypt")
            .help("Use scrypt instead of PBKDF2 in the BIP39 derivation. This makes keys harder to brute force, but it deviates from the BIP39 standard.")
        )
        .get_matches();

    let coin_type = value_t!(args, "coin", CoinType).unwrap_or_else(|e| e.exit());
    let scrypt = args.is_present("scrypt");
    let mnemonic = args.value_of("from-mnemonic");
    let mnemonic: Box<dyn Mnemonic> = if scrypt {
        Box::new(
            mnemonic
                .map(|m| ScryptMnemonic::from_phrase(m))
                .unwrap_or_else(|| ScryptMnemonic::generate())?,
        )
    } else {
        Box::new(
            mnemonic
                .map(|m| Bip39Mnemonic::from_phrase(m))
                .unwrap_or_else(|| Bip39Mnemonic::generate())?,
        )
    };
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

    if scrypt {
        print!("Generating keys with scrypt. This can take a while...");
        io::stdout().lock().flush().expect("Flushing stdout failed");
    }
    let master_key = mnemonic.to_private_key(&password1)?;
    if scrypt {
        println!("done");
    }
    let derived = derive_key(master_key, coin_type, account_index)?;
    match coin_type {
        CoinType::XMR => {
            let wallet = MoneroWallet::from_hd_key(derived)?;

            println!(
                "Mnemonic: {}\nPassword: [omitted]\nAddress: {}\nPrivate View Key: {}\nPrivate Spend Key: {}",
                mnemonic.phrase(),
                wallet.address()?,
                wallet.private_view_key(),
                wallet.private_spend_key(),
            );
        }
        CoinType::BTC => {
            let wallet = BitcoinWallet::from_hd_key(derived)?;

            println!(
                "Mnemonic: {}\nPassword: [omitted]\nPrivate Key: {}",
                mnemonic.phrase(),
                wallet.private_key(),
            );
        }
    }

    Ok(())
}

fn derive_key(master_key: HDPrivKey, coin_type: CoinType, account: u32) -> Result<HDPrivKey> {
    master_key.derive(Bip44DerivationPath {
        coin_type,
        account,
        // Don't derive change and address_index, this is up to the wallet software.
        // Doing it this way means we can directly import our private key into electrum
        // and it will match the BIP44 standard.
        change: None,
        address_index: None,
    })
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
            derive_key(master_seed, CoinType::BTC, 0).unwrap().to_base58(),
        );
        // and loaded that key into electrum, checking that electrum generates the BIP44 addresses
        // listed on https://iancoleman.io/bip39/
        // So this test case is basically a test ensuring that we keep generating the same private key for which we already checked
        // what electrum generates from it and don't start differring from it.
    }
}
