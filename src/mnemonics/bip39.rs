use anyhow::Result;
use bip39::{Language, Mnemonic as _Mnemonic, Seed as _Seed};
use rand::RngCore;

use super::{Mnemonic, MnemonicFactory};
use crate::bip32::HDPrivKey;
use crate::random::secure_rng;
use crate::seed::Seed;

const LANG: Language = Language::English;

#[derive(Debug)]
pub struct Bip39Mnemonic {
    // wagyu_bitcoin::mnemonic::BitcoinMnemonic::to_seed() is private, so we need to use the bip39 crate instead.
    mnemonic: _Mnemonic,
}

impl MnemonicFactory for Bip39Mnemonic {
    fn generate() -> Result<Self> {
        const ENTROPY_LENGTH: usize = 32;
        // XOR an OS rng and a pseudo rng to get our entropy. Probably not necessary but doesn't hurt either.
        let mut rng = secure_rng()?;
        let mut entropy: [u8; ENTROPY_LENGTH] = [0; ENTROPY_LENGTH];
        rng.fill_bytes(&mut entropy);
        let mnemonic = _Mnemonic::from_entropy(&entropy, LANG).expect("Invalid key length");
        Ok(Self { mnemonic })
    }

    fn from_phrase(phrase: &str) -> Result<Self> {
        let mnemonic = _Mnemonic::from_phrase(phrase, LANG)?;
        Ok(Self { mnemonic })
    }

    fn validate(phrase: &str) -> Result<()> {
        _Mnemonic::validate(phrase, LANG)
    }
}

impl Mnemonic for Bip39Mnemonic {
    fn phrase(&self) -> &str {
        self.mnemonic.phrase()
    }

    fn into_phrase(self) -> String {
        self.mnemonic.into_phrase()
    }

    fn to_private_key(&self, password: &str) -> Result<HDPrivKey> {
        let seed = Seed::from_bytes(_Seed::new(&self.mnemonic, password).as_bytes().to_vec());
        HDPrivKey::new(seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_generated_key_is(expected_key: &str, phrase: &str, password: &str) {
        assert_eq!(
            expected_key,
            Bip39Mnemonic::from_phrase(phrase)
                .unwrap()
                .to_private_key(password)
                .unwrap()
                .to_base58()
        );
    }

    #[test]
    fn twelve_words_without_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_key_is(
            "xprv9s21ZrQH143K2cidnrzfWcHRJ23QxfAEoFdVkBgbT9mns2FPMBWZwnXZbhXsVXgSzmE2JqHmVhAna7E7L6WQ6DKagT3f6fA6bwVwkWtaSLp",
            "lunch blanket cruise chair question good market allow blue celery little void",
            "");
    }

    #[test]
    fn twelve_words_with_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_key_is(
            "xprv9s21ZrQH143K3wy3DhgTQ44zJb99zRLbhtrp6t3pitm9jTwaFMghhdNosoeCTy7GDJSSh3F9aenvk6WQDAU37yhqTHybANPvLgAE9s9vL7X",
            "lunch blanket cruise chair question good market allow blue celery little void",
            "my password");
    }

    #[test]
    fn fifteen_words_without_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_key_is(
            "xprv9s21ZrQH143K3zBjoLR71dBPE3pKi62h97rKgh5J6TdveEMFB71MukBF12jB8vWhXzV8DYbxL9V3PqdRQBsKkYtjf3BZonWcV7WHvByhpk3",
            "mirror distance build unaware current concert link chapter resemble tuition main rent echo drum dolphin",
            "");
    }

    #[test]
    fn fifteen_words_with_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_key_is(
            "xprv9s21ZrQH143K4a1FCVYWCbiLVFXj2m9k2MwU19Kc7nFzyFzXLnRV2Ka5pNT4Tw1DPMXWjXSFbZbzvpv9MGDMfTuiMUCnSATsaq8gA5kfERZ",
            "mirror distance build unaware current concert link chapter resemble tuition main rent echo drum dolphin",
            "my password");
    }

    #[test]
    fn eighteen_words_without_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_key_is(
            "xprv9s21ZrQH143K4SCLnE8JFuAe7q83dNbnd1VhH7pLchL5wXYQRg9bJguPcX8fCTDWiMndRLt7FCZA9zozQCKGn5CnCbx3zErw48XvYEnMTvg",
            "blush section drift canoe reform friend rose cherry assume supreme home hub goat arena jazz absurd emotion hidden",
            "");
    }

    #[test]
    fn eighteen_words_with_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_key_is(
            "xprv9s21ZrQH143K3hzQrTYxsE8ASRQnymatPj7QnK83E9yL7c7ynLU4kx6LN7MCpy9vwei6stthAh6nBB8TmWxDr7FssJMGt2YN3jfT9Ksj6ih",
            "blush section drift canoe reform friend rose cherry assume supreme home hub goat arena jazz absurd emotion hidden",
            "my password");
    }

    #[test]
    fn twentyone_words_without_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_key_is(
            "xprv9s21ZrQH143K3LDb5bbmmEHpowwV9JgcSJ7nJPmiNCMbS2EisLt1iHXrYnWubffdpCgTgKR4Km6VVrPwgf4TgSzD4QNpgJ3L1cAAEEeVuw7",
            "include disagree sentence junior gospel engage whip old boost scrap someone helmet list best afraid favorite gold antenna before peasant buffalo",
            "");
    }

    #[test]
    fn twentyone_words_with_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_key_is(
            "xprv9s21ZrQH143K4MHsTjvdG9QEjbKwrZGKsjNxxCSwkDjnVM91M6d4e5XR2bnva5GNgSf2pdvg9JubTa9UMNEDisAKD6Dg7DW74xPgr91KcNA",
            "include disagree sentence junior gospel engage whip old boost scrap someone helmet list best afraid favorite gold antenna before peasant buffalo",
            "my password");
    }

    #[test]
    fn twentyfour_words_without_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_key_is(
            "xprv9s21ZrQH143K3ss3HXZFVjYApfQkdokhpiGjpXnm8y8sfbtb4ydSwsPXUSj7g1mY8VhJH3iY1ZUrgdbcFmvmEhzq6R35WW4JNBSZz4uLCXN",
            "table car outdoor twist dutch auction monitor rude pumpkin very disease ability hope area metal brisk luggage tell ribbon profit various lake topic exist",
            "");
    }

    #[test]
    fn twentyfour_words_with_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_key_is(
            "xprv9s21ZrQH143K4XjwHvT3EEwu2fc9T3YVyXTq96SUnpRviKA49y1Lf4UxPd3t5DNRj6xffnhZM2pRVYr3BjUCQ8RCvJEWxQqUBeTRWKuNqp2",
            "table car outdoor twist dutch auction monitor rude pumpkin very disease ability hope area metal brisk luggage tell ribbon profit various lake topic exist",
            "my password");
    }

    #[test]
    fn generated_phrase_is_24_words() {
        let phrase = Bip39Mnemonic::generate().unwrap().into_phrase();
        assert_eq!(23, phrase.chars().filter(|a| *a == ' ').count());
    }

    #[test]
    fn generated_phrase_is_valid() {
        Bip39Mnemonic::validate(Bip39Mnemonic::generate().unwrap().phrase()).unwrap();
    }

    #[test]
    fn validate_valid_24word_phrase() {
        Bip39Mnemonic::validate("desert armed renew matrix congress order remove lab travel shallow there tool symbol three radio exhibit pledge alcohol quit host rare noble dose eager").unwrap();
    }

    #[test]
    fn validate_valid_21word_phrase() {
        Bip39Mnemonic::validate("morning mind present cloud boat phrase task uniform effort couple carpet wise steak eyebrow friend birth million photo tobacco firm hobby").unwrap();
    }

    #[test]
    fn validate_valid_18word_phrase() {
        Bip39Mnemonic::validate("slice lift violin movie shield copy tail arrow idle lift knock fossil leave lawsuit tennis sight travel vivid").unwrap();
    }

    #[test]
    fn validate_valid_15word_phrase() {
        Bip39Mnemonic::validate("call oval opinion exhibit limit write fine prepare sleep possible extend language split kidney desert").unwrap();
    }

    #[test]
    fn validate_valid_12word_phrase() {
        Bip39Mnemonic::validate(
            "tornado ginger error because arrange lake scale unfold palm theme frozen sick",
        )
        .unwrap();
    }

    #[test]
    fn validate_invalid_20word_phrase() {
        let err = Bip39Mnemonic::validate(
            "morning mind present cloud boat phrase task uniform effort couple carpet wise steak eyebrow friend birth million photo tobacco firm",
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("invalid number of words in phrase"))
    }

    #[test]
    fn validate_invalid_21word_phrase() {
        let err = Bip39Mnemonic::validate(
            "morning mind present cloud boat phrase task uniform effort couple carpet wise steak eyebrow friend birth million photo tobacco firm prepare",
        )
        .unwrap_err();
        assert!(err.to_string().contains("invalid checksum"))
    }

    #[test]
    fn from_invalid_20word_phrase() {
        let err = Bip39Mnemonic::from_phrase(
            "morning mind present cloud boat phrase task uniform effort couple carpet wise steak eyebrow friend birth million photo tobacco firm",
        )
        .unwrap_err();
        assert!(err
            .to_string()
            .contains("invalid number of words in phrase"))
    }

    #[test]
    fn from_invalid_21word_phrase() {
        let err = Bip39Mnemonic::from_phrase(
            "morning mind present cloud boat phrase task uniform effort couple carpet wise steak eyebrow friend birth million photo tobacco firm prepare",
        )
        .unwrap_err();
        assert!(err.to_string().contains("invalid checksum"))
    }
}
