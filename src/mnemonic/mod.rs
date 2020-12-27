use anyhow::Result;

use crate::seed::Seed;

pub trait Mnemonic: Sized {
    fn generate() -> Self;

    fn phrase(&self) -> &str;
    fn into_phrase(self) -> String;
    fn from_phrase(phrase: &str) -> Result<Self>;

    fn to_seed(&self, password: &str) -> Seed;
}

pub mod bip39;
