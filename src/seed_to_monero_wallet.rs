use anyhow::Result;
use failure::Fail;
use wagyu_monero::format::MoneroFormat;
use wagyu_monero::network::mainnet::Mainnet;
use wagyu_monero::private_key::MoneroPrivateKey;
use wallet_gen::coin::Coin;
use wallet_gen::cryptonote;
use wallet_gen::wallet::Wallet;

use crate::seed::Seed;

pub trait MoneroWallet {
    fn address(&self) -> &str;
    fn private_spend_key(&self) -> &str;
    fn public_spend_key(&self) -> &str;
    fn private_view_key(&self) -> Result<String>;
}

impl MoneroWallet for Wallet {
    fn address(&self) -> &str {
        &self.address
    }

    fn private_spend_key(&self) -> &str {
        &self.private_key
    }

    fn public_spend_key(&self) -> &str {
        &self.public_key
    }

    fn private_view_key(&self) -> Result<String> {
        let key = MoneroPrivateKey::<Mainnet>::from_private_spend_key(
            self.private_spend_key(),
            &MoneroFormat::Standard,
        )
        .map_err(|err| err.compat())?;
        Ok(hex::encode(key.to_private_view_key()))
    }
}

pub fn seed_to_monero_wallet(seed: impl Seed) -> Result<impl MoneroWallet> {
    let mut seed_arr = [0; 32];
    seed_arr.clone_from_slice(seed.as_bytes()); // TODO Make Seed type work with fixed size arrays instead of this?
    let wallet = cryptonote::from_seed(Coin::Monero, seed_arr)?;
    Ok(wallet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seed;

    #[test]
    fn example1() {
        // Randomly generated on https://xmr.llcoins.net/addresstests.html
        let seed =
            seed::from_hex("177c328073abe1486ceb190ee4ef544896f2ff0fe6b1c83d28de2cc68d22b106")
                .unwrap();
        let wallet = seed_to_monero_wallet(seed).unwrap();
        assert_eq!(
            "177c328073abe1486ceb190ee4ef544896f2ff0fe6b1c83d28de2cc68d22b106",
            wallet.private_spend_key(),
        );
        assert_eq!(
            "946f666fd47ba8c0c0f564ec3aea442f4e5d121fe35e00c63056daa6ee93fb7a",
            wallet.public_spend_key(),
        );
        assert_eq!(
            "08b6eeff17cc5a66054b83d6ad710d8894100a6c672925ecc49cf2521af4c206",
            wallet.private_view_key().unwrap(),
        );
        assert_eq!("47FMqqLkqTVZExG8eJg5hV8uvrUvffjQsa9gS59tLiVxMWtAZH4SULSMhDnPiZDe4bUtGRv3wq7wcER8HymBEeDyDoXyvPa", wallet.address());
    }

    #[test]
    fn example2() {
        // Randomly generated on https://xmr.llcoins.net/addresstests.html
        let seed =
            seed::from_hex("786dbcf5c283165f77445327ddaf44a05104d54eb4e5920da776d1a844b20703")
                .unwrap();
        let wallet = seed_to_monero_wallet(seed).unwrap();
        assert_eq!(
            "786dbcf5c283165f77445327ddaf44a05104d54eb4e5920da776d1a844b20703",
            wallet.private_spend_key(),
        );
        assert_eq!(
            "c98e3bcbb80566d7b1fa9d4d02b4d1e6644cc322f820868dc5e528e175262183",
            wallet.public_spend_key(),
        );
        assert_eq!(
            "17b4eda6613ded666609fcc3a88d2a27336734fe50f6766f917cccf5715ff704",
            wallet.private_view_key().unwrap(),
        );
        assert_eq!("49G7fW8KGG5d5WoqvjGBUtfY6AUmRSfJmQiNojwGYgCYP36TtVKf4ZgNPf3V15Mf1oB3QT745Hmop2acHnWrC86tJJGhaEi", wallet.address());
    }
}
