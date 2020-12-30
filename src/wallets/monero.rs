use anyhow::{anyhow, Result};
use failure::Fail;
use wagyu_model::private_key::PrivateKey;
use wagyu_monero::format::MoneroFormat;
use wagyu_monero::network::mainnet::Mainnet;
use wagyu_monero::private_key::MoneroPrivateKey;

use super::Wallet;
use crate::bip32::HDPrivKey;
use crate::seed::Seed;

pub struct MoneroWallet {
    private_key: MoneroPrivateKey<Mainnet>,
}

impl MoneroWallet {
    pub fn from_seed(seed: &Seed) -> Result<Self> {
        Ok(Self {
            private_key: MoneroPrivateKey::from_seed(
                &hex::encode(seed.to_bytes()),
                &MoneroFormat::Standard,
            )
            .map_err(|err| err.compat())?,
        })
    }

    pub fn address(&self) -> Result<String> {
        Ok(format!(
            "{}",
            self.private_key
                .to_address(&MoneroFormat::Standard)
                .map_err(|err| err.compat())?
        ))
    }

    pub fn private_spend_key(&self) -> String {
        hex::encode(self.private_key.to_private_spend_key())
    }

    pub fn public_spend_key(&self) -> Result<String> {
        Ok(hex::encode(
            self.private_key
                .to_public_key()
                .to_public_spend_key()
                .ok_or_else(|| anyhow!("Couldn't calculate public spend key"))?,
        ))
    }

    pub fn private_view_key(&self) -> String {
        hex::encode(self.private_key.to_private_view_key())
    }
}

impl Wallet for MoneroWallet {
    fn from_hd_key(private_key: HDPrivKey) -> Result<Self> {
        Self::from_seed(&private_key.key_part())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        // Randomly generated on https://xmr.llcoins.net/addresstests.html
        let seed =
            Seed::from_hex("177c328073abe1486ceb190ee4ef544896f2ff0fe6b1c83d28de2cc68d22b106")
                .unwrap();
        let wallet = MoneroWallet::from_seed(&seed).unwrap();
        assert_eq!(
            "177c328073abe1486ceb190ee4ef544896f2ff0fe6b1c83d28de2cc68d22b106",
            wallet.private_spend_key(),
        );
        assert_eq!(
            "946f666fd47ba8c0c0f564ec3aea442f4e5d121fe35e00c63056daa6ee93fb7a",
            wallet.public_spend_key().unwrap(),
        );
        assert_eq!(
            "08b6eeff17cc5a66054b83d6ad710d8894100a6c672925ecc49cf2521af4c206",
            wallet.private_view_key(),
        );
        assert_eq!("47FMqqLkqTVZExG8eJg5hV8uvrUvffjQsa9gS59tLiVxMWtAZH4SULSMhDnPiZDe4bUtGRv3wq7wcER8HymBEeDyDoXyvPa", wallet.address().unwrap());
    }

    #[test]
    fn example2() {
        // Randomly generated on https://xmr.llcoins.net/addresstests.html
        let seed =
            Seed::from_hex("786dbcf5c283165f77445327ddaf44a05104d54eb4e5920da776d1a844b20703")
                .unwrap();
        let wallet = MoneroWallet::from_seed(&seed).unwrap();
        assert_eq!(
            "786dbcf5c283165f77445327ddaf44a05104d54eb4e5920da776d1a844b20703",
            wallet.private_spend_key(),
        );
        assert_eq!(
            "c98e3bcbb80566d7b1fa9d4d02b4d1e6644cc322f820868dc5e528e175262183",
            wallet.public_spend_key().unwrap(),
        );
        assert_eq!(
            "17b4eda6613ded666609fcc3a88d2a27336734fe50f6766f917cccf5715ff704",
            wallet.private_view_key(),
        );
        assert_eq!("49G7fW8KGG5d5WoqvjGBUtfY6AUmRSfJmQiNojwGYgCYP36TtVKf4ZgNPf3V15Mf1oB3QT745Hmop2acHnWrC86tJJGhaEi", wallet.address().unwrap());
    }

    #[test]
    fn regression1() {
        // This is a regression test. This special case of a key with trailing zeroes caused the key derivation of
        // the wallet-gen crate to fail. See https://gitlab.com/standard-mining/wallet-gen/-/issues/1
        let seed =
            Seed::from_hex("6734c05d337c2f4883eb710bc02be1c30f1b2d46b2657c46cc833eecb7d7cb10")
                .unwrap();
        let wallet = MoneroWallet::from_seed(&seed).unwrap();
        assert_eq!(
            "7a60ca0019191df0ac4e7a68e13102af0f1b2d46b2657c46cc833eecb7d7cb00",
            wallet.private_spend_key(),
        );
        assert_eq!(
            "cb778d7f9fbe165be14a255640745eda8625276469e51659759caf6b3c048b1c",
            wallet.public_spend_key().unwrap(),
        );
        assert_eq!(
            "f5467d54c558a8a34b5f7bdd51a032fbe95a92e242133780adcd29df5d87da00",
            wallet.private_view_key(),
        );
        assert_eq!("49LKLAixdiuGNMPJne3E7odYxUvgzhGA1FxsNV6zeAUr5nCUXyjUXLugNiMRMiCnZUAck57e5xHE58wiwmtfAfxrTwzkrkX", wallet.address().unwrap());
    }
}
