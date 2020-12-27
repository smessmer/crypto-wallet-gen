use anyhow::Result;

use crate::seed::Seed;

pub trait Wallet: Sized {
    fn from_seed(seed: &Seed) -> Result<Self>;
}

pub mod bitcoin;
pub mod monero;
