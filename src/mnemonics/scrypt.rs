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
    // Using parameters that are higher than the ones proposed in BIP38
    // (note log2(N) == 21 means N == 2097152)
    let scrypt_params = ScryptParams::new(21, 8, 8).expect("Invalid hardcoded scrypt params");
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
            "xprv9s21ZrQH143K43yeDzy9ERFAT7BvsnetT7Z45fgWycciwUYrkarQrH9BheSZBp9pW24DT3XAnwTWHVZPc5KmejWrqnEpnMp5pK2HX4GZFwg",
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
            "xprv9s21ZrQH143K2hGy14dk1o5oSEvcGqvD6UTVXwXKQjFTH688RpFVrkpFx34Kbn7CLFXPkthL59TfGAs5RZepXyB1TSK4n85rjaxoeH22krV",
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
            "xprv9s21ZrQH143K2nJNnAc8Pipt7rcXfPPtsH44wZghrhZK2X6j66JFR6zJcRhhbZsPdqQiMDuhsUqheQYxoCyJnZnCbDJ8YXHnqnykpiuD9LL",
            "mirror distance build unaware current concert link chapter resemble tuition main rent echo drum dolphin",
            "");
    }

    #[test]
    fn fifteen_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K42m6cRxBUT3Yij6vVdPKo5JPxStd3SLFEBD5e2xDv9M1mox6EKPFyfKpJFdtsPCN1Rt3SfrhFBfGSnDzxHQRje9RdPK29Ue",
            "mirror distance build unaware current concert link chapter resemble tuition main rent echo drum dolphin",
            "my password");
    }

    #[test]
    fn eighteen_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K3chwjAuMLiKUQf4SdtgYzb47X64rGH86AEQZoyFk9uyEhtN8LD7SpFQcGAro9SuvTyMRASGaiL4qDBDyvUUw9upmvFqdZaK",
            "blush section drift canoe reform friend rose cherry assume supreme home hub goat arena jazz absurd emotion hidden",
            "");
    }

    #[test]
    fn eighteen_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K3uAtjMM2GJNP3SLef2tNCXRkYZWsUh7biyvCvmcHTWvpKoV1PJPcr1RRabFGYTQLu6WQqTV4vc7dhhZEkdUP4LGzYveCZ1r",
            "blush section drift canoe reform friend rose cherry assume supreme home hub goat arena jazz absurd emotion hidden",
            "my password");
    }

    #[test]
    fn twentyone_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K2SuSTDXTTspsstHYeUuhifsLeuXSMnd6LUYy9ogKumvJYNPsWYXwVMHAfK5rd143UV2KQn9kJQ6bhgiyXrux7AAP6nBK8H8",
            "include disagree sentence junior gospel engage whip old boost scrap someone helmet list best afraid favorite gold antenna before peasant buffalo",
            "");
    }

    #[test]
    fn twentyone_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K2iBfhzqqbUstbgAkvTizaRCL2KrUUUEZTL5bt2VBry9R8s8Ru29NGTYftwfKSY56623jXiQ2TygvcUA35ntm62q6GFKjvGe",
            "include disagree sentence junior gospel engage whip old boost scrap someone helmet list best afraid favorite gold antenna before peasant buffalo",
            "my password");
    }

    #[test]
    fn twentyfour_words_without_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K2VmVFn8LU6ZgJEEBwPTSsYrv91gTJhvMo7yDPwhDxifS14FK6JqVPtKiiYPyJpUjHN3XqyGRZaBELfERRBt9PXbL3VvsXsU",
            "table car outdoor twist dutch auction monitor rude pumpkin very disease ability hope area metal brisk luggage tell ribbon profit various lake topic exist",
            "");
    }

    #[test]
    fn twentyfour_words_with_password() {
        // Since there is no online sources for our scrypt approach, this was generated with
        // our own algorithm and is more a regression test to make sure we don't accidentally
        // change the algorithm.
        expect_generated_key_is(
            "xprv9s21ZrQH143K2kzr4K5S2nc2Kg7n5wnNG2kbTpdrLpu1fhFCwrcp8SRJKbcSzBMa7p6bfyzsy3b1EeY6HNpu78xbdCQY4Yj3t3f2doXbSw1",
            "table car outdoor twist dutch auction monitor rude pumpkin very disease ability hope area metal brisk luggage tell ribbon profit various lake topic exist",
            "my password");
    }

    #[test]
    fn generated_phrase_is_24_words() {
        let phrase = ScryptMnemonic::generate().unwrap().into_phrase();
        assert_eq!(23, phrase.chars().filter(|a| *a == ' ').count());
    }
}
