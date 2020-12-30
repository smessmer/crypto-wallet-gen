use ::bitcoin::util::bip32::ExtendedPrivKey;
use anyhow::Result;

pub mod bitcoin;
pub mod monero;

pub trait Wallet: Sized {
    fn from_extended_key(private_key: ExtendedPrivKey) -> Result<Self>;
}
