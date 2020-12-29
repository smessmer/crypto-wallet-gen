use anyhow::Result;

use crate::hd_seed::HDSeed;

pub trait Mnemonic: Sized {
    fn generate() -> Result<Self>;

    fn phrase(&self) -> &str;
    fn into_phrase(self) -> String;
    fn from_phrase(phrase: &str) -> Result<Self>;

    fn to_seed(&self, password: &str) -> HDSeed;
}

pub mod bip39;
