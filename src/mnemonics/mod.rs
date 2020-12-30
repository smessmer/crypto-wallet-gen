use anyhow::Result;

use crate::bip32::HDPrivKey;

pub trait MnemonicFactory: Sized {
    fn generate() -> Result<Self>;
    fn from_phrase(phrase: &str) -> Result<Self>;

    /// Validate a mnemonic phrase
    ///
    /// The phrase supplied will be checked for word length and validated according to the checksum
    /// specified in BIP0039.
    fn validate(phrase: &str) -> Result<()>;
}

pub trait Mnemonic {
    fn phrase(&self) -> &str;
    fn into_phrase(self) -> String;
    fn to_private_key(&self, password: &str) -> Result<HDPrivKey>;
}

pub mod bip39;
pub mod scrypt;
