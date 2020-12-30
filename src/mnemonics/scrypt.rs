use anyhow::Result;
use scrypt::{scrypt, ScryptParams};
use unicode_normalization::UnicodeNormalization;

use super::bip39::Bip39Mnemonic;
use super::{Mnemonic, MnemonicFactory};
use crate::bip32::HDPrivKey;
use crate::seed::Seed;

/// A mnemonic similar to BIP39, but using scrypt instead of PBKDF2 for the key derivation.
pub struct ScryptMnemonic {
    phrase: String,
}

impl MnemonicFactory for ScryptMnemonic {
    fn generate() -> Result<Self> {
        Ok(Self {
            phrase: Bip39Mnemonic::generate()?.into_phrase(),
        })
    }

    fn from_phrase(phrase: &str) -> Result<Self> {
        Self::validate(phrase)?;
        Ok(Self {
            phrase: phrase.to_string(),
        })
    }

    fn validate(phrase: &str) -> Result<()> {
        Bip39Mnemonic::validate(phrase)
    }
}

impl Mnemonic for ScryptMnemonic {
    fn phrase(&self) -> &str {
        &self.phrase
    }

    fn into_phrase(self) -> String {
        self.phrase
    }

    fn to_private_key(&self, password: &str) -> Result<HDPrivKey> {
        let salt = format!("mnemonic{}", password);
        let normalized_salt = salt.nfkd().to_string();
        let bytes = kdf(self.phrase.as_bytes(), normalized_salt.as_bytes())?;

        HDPrivKey::new(Seed::from_bytes(bytes))
    }
}

fn kdf(password: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
    // Using parameters proposed in BIP38
    // (note log2(N) == 14 means N == 16384)
    let scrypt_params = ScryptParams::new(14, 8, 8).expect("Invalid hardcoded scrypt params");
    const OUTPUT_BYTES: usize = 64;
    let mut seed = vec![0u8; OUTPUT_BYTES];
    scrypt(password, salt, &scrypt_params, &mut seed)?;

    Ok(seed)
}

// TODO Test validate
// TODO Test that from_phrase rejects invalid phrases

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_generated_key_is(expected_key: &str, phrase: &str, password: &str) {
        assert_eq!(
            expected_key,
            ScryptMnemonic::from_phrase(phrase)
                .unwrap()
                .to_private_key(password)
                .unwrap()
                .to_base58()
        );
    }

    #[test]
    fn twelve_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K3J7pMib2p2S7y7UeZtcztzJxWNFTPNd2E8Q4Z4uCfme3zvy8Dzm4hNV1ie7utKmfMVYkS29QGJxqZVtZzBK7jLtmXttnsxF",
            "lunch blanket cruise chair question good market allow blue celery little void",
            "",
        );
    }

    #[test]
    fn twelve_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K2uBmVRhkXdaBmCoLSXnFCs8qudvDZjdJHdJUQZCqmJ9etAhVAyibkApgHFXgZStauDvkUM4ascX2xQGMh2EfvPTP9czhqf9",
            "lunch blanket cruise chair question good market allow blue celery little void",
            "my password",
        );
    }

    #[test]
    fn fifteen_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K2vEMFXiGpezMvvyxiYoEWnWjsvm1ty5M8w7BzX3wa3wzmtF3udu6md4K9dU8xcygMeT7Ay9sYJSWEceBZ6zdZzZFCkPaodB",
            "mirror distance build unaware current concert link chapter resemble tuition main rent echo drum dolphin",
            "");
    }

    #[test]
    fn fifteen_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K29GuP4S6Zf3MZjHnXTB13Qo5Z6jEwj7oAJaRV9S3f3FdK7pVRpgAdi1cEpT9sNwsxYLwUKMGcqEh2LZGuksz1cozfrPUTRz",
            "mirror distance build unaware current concert link chapter resemble tuition main rent echo drum dolphin",
            "my password");
    }

    #[test]
    fn eighteen_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K4D54Us4Sfi3fkPJLCoEP9swC1nqeGKywPmhZnfdbt1YZrCiFZxBdqM6GdScvk1cT7eyrpguWP8rxQ2rC1XRZ5XtHbfkRSB9",
            "blush section drift canoe reform friend rose cherry assume supreme home hub goat arena jazz absurd emotion hidden",
            "");
    }

    #[test]
    fn eighteen_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K2wAnUzRhbkrk6UaqXPSkabnVJ6fbkFUbXrX5TF2nreDJFQJ17AnbqPzo4QBitM1bY3mrwJGvP7crWjoN2CnRnEVCqmr6i4d",
            "blush section drift canoe reform friend rose cherry assume supreme home hub goat arena jazz absurd emotion hidden",
            "my password");
    }

    #[test]
    fn twentyone_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K45kY6cgVNyydTFxsgJSUMTPfgb9HZWi16G4qL37ExxjQH7qZH3UHVCbT1Q54muYDoMrdoAgwG9j4anFcEYmifvLBBY4fz1W",
            "include disagree sentence junior gospel engage whip old boost scrap someone helmet list best afraid favorite gold antenna before peasant buffalo",
            "");
    }

    #[test]
    fn twentyone_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K2Rp3kxxRvT2tTpaLzYyZWuULixPh4VnRjoDEXwwPYKNdHYzUsnxgwoi2H1u1JahpaXKL5f9vQTYJ4jr8qVT9wKgwiwRnoME",
            "include disagree sentence junior gospel engage whip old boost scrap someone helmet list best afraid favorite gold antenna before peasant buffalo",
            "my password");
    }

    #[test]
    fn twentyfour_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K41URuZrC5bfU2kbMrAgVW1VQktsxoDUUg16m6R8pExSzNW7XHhmr7QwWn8M5x5BYcfyumkCPTM4Nuwqz5wwXRQF6YYiF8d7",
            "table car outdoor twist dutch auction monitor rude pumpkin very disease ability hope area metal brisk luggage tell ribbon profit various lake topic exist",
            "");
    }

    #[test]
    fn twentyfour_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K436egABGFTdjxMPa4fEgAYvrT3LcsK8zvSGEr1jC4v8egWNeiJdqrr8TcHAbWdUrMQY81wiHWJeoxECfSQaEh3RUhhSHZLR",
            "table car outdoor twist dutch auction monitor rude pumpkin very disease ability hope area metal brisk luggage tell ribbon profit various lake topic exist",
            "my password");
    }

    #[test]
    fn generated_phrase_is_24_words() {
        let phrase = ScryptMnemonic::generate().unwrap().into_phrase();
        assert_eq!(23, phrase.chars().filter(|a| *a == ' ').count());
    }
}
