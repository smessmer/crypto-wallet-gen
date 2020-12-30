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
    const OUTPUT_BYTES: usize = 64;
    let mut seed = vec![0u8; OUTPUT_BYTES];
    scrypt(password, salt, &scrypt_params(), &mut seed)?;

    Ok(seed)
}

#[cfg(test)]
fn scrypt_params() -> ScryptParams {
    // Tests need lower scrypt params or they won't be able to run on CI machines
    ScryptParams::new(12, 1, 1).expect("Invalid hardcoded scrypt params")
}

#[cfg(not(test))]
fn scrypt_params() -> ScryptParams {
    // Using parameters that are higher than the ones proposed in BIP38
    // (note log2(N) == 21 means N == 2097152)
    ScryptParams::new(21, 8, 8).expect("Invalid hardcoded scrypt params")
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
            "xprv9s21ZrQH143K31h69CVTU374efVBSbx8PHnh27om2e7Nh4r8wjvnrb3iHrH4HWn1KVUM27YEf5UtaZt7AKvv7HBjhkmSdnoWYpVNSqQHXMK",
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
            "xprv9s21ZrQH143K3KrGus5NXedDmE1MHgRhy5Kpa1fsiRm3PeG6bE4oxgqRAuxFHqMPMcEKrALFKmVpMj6jAzbTaEncJSUqUCWFQdMh4njQN7X",
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
            "xprv9s21ZrQH143K371jBvAZqkzZoXsLVNPWVtCPbyqKBtwEDY31vXqNkGuYqmJnxfPUkzSgQ4MC2BAFchkAYAirRek7BejSt59hfpnnTeGVNzS",
            "mirror distance build unaware current concert link chapter resemble tuition main rent echo drum dolphin",
            "");
    }

    #[test]
    fn fifteen_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K2AqPnXvRcDw5ypxw5BpwxhuWnbeaiQwB5RueZsKZqB1TZGpBrtWiM3dGHr8BJtPMc4jTG7bDgsp2LXgQFgtDkiXxYmaArKj",
            "mirror distance build unaware current concert link chapter resemble tuition main rent echo drum dolphin",
            "my password");
    }

    #[test]
    fn eighteen_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K3zCCiVgq3MAthXj1BLaD4CZa4UJXH3yttQWvXUGjMoR94eHeNLbgHpPJTQ5ayw73ng98QCXifABhnYenU73U1YvnaBt3fc7",
            "blush section drift canoe reform friend rose cherry assume supreme home hub goat arena jazz absurd emotion hidden",
            "");
    }

    #[test]
    fn eighteen_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K4EWgn4SVWUrJKziE8n7qSbPC94wNRbupQXk6acDSAgv4kbBhXRqCTuspABiijrrzabcmKH14mMymF3t4uJk8MRhSogB9vjf",
            "blush section drift canoe reform friend rose cherry assume supreme home hub goat arena jazz absurd emotion hidden",
            "my password");
    }

    #[test]
    fn twentyone_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K3RaUETg9duZwV5CtwsKwV2BRjy1e5CWCLt8YQHrFCTic42gAhfL91NidSJfpmie8YWMycpMRPrMLAC87hrDjvgreCRDbrBu",
            "include disagree sentence junior gospel engage whip old boost scrap someone helmet list best afraid favorite gold antenna before peasant buffalo",
            "");
    }

    #[test]
    fn twentyone_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K46Wg1D47KYpxFsZWBsm9Xth7AJUgHwCAd2iKLowwbHK56JDBVtiyya2q4TScLAS8NvE81aZtN3GFbm3exeXjKdATmBAfz6e",
            "include disagree sentence junior gospel engage whip old boost scrap someone helmet list best afraid favorite gold antenna before peasant buffalo",
            "my password");
    }

    #[test]
    fn twentyfour_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K3jgRiJbM3phUCscqjNpU7VSedfquJ9BeW2DdmMaksZvf3cjMFMfhPqgxNtMxhZgjQyzDSvQq8ASTQqcPN5pkiKCbf59rAt8",
            "table car outdoor twist dutch auction monitor rude pumpkin very disease ability hope area metal brisk luggage tell ribbon profit various lake topic exist",
            "");
    }

    #[test]
    fn twentyfour_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K2wDqEuYRrXbVruhDgcVMe4fSqYMjny7shxkLUe2HLxSQNFvUKt3VA68v2q43UXSPAjMTdRV7DEN5bo4hCV8wvbbaHhDxNAK",
            "table car outdoor twist dutch auction monitor rude pumpkin very disease ability hope area metal brisk luggage tell ribbon profit various lake topic exist",
            "my password");
    }

    #[test]
    fn generated_phrase_is_24_words() {
        let phrase = ScryptMnemonic::generate().unwrap().into_phrase();
        assert_eq!(23, phrase.chars().filter(|a| *a == ' ').count());
    }
}
