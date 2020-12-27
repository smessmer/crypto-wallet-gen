use anyhow::Result;
use bip39::{Language, Mnemonic as _Mnemonic, Seed as _Seed};
use rand::{rngs::OsRng, thread_rng, RngCore};

use super::Mnemonic;
use crate::hd_seed::HDSeed;
use crate::seed::Seed;

const LANG: Language = Language::English;

pub struct Bip39Mnemonic {
    mnemonic: _Mnemonic,
}

impl Mnemonic for Bip39Mnemonic {
    fn generate() -> Self {
        const ENTROPY_LENGTH: usize = 32;
        // XOR an OS rng and a pseudo rng to get our entropy. Probably not necessary but doesn't hurt either.
        let mut prng_entropy: [u8; ENTROPY_LENGTH] = [0; ENTROPY_LENGTH];
        thread_rng().fill_bytes(&mut prng_entropy);
        let mut entropy: [u8; ENTROPY_LENGTH] = [0; ENTROPY_LENGTH];
        OsRng.fill_bytes(&mut entropy);
        for i in 0..ENTROPY_LENGTH {
            entropy[i] ^= prng_entropy[i];
        }
        let mnemonic = _Mnemonic::from_entropy(&entropy, LANG).expect("Invalid key length");
        Self { mnemonic }
    }

    fn phrase(&self) -> &str {
        self.mnemonic.phrase()
    }

    fn into_phrase(self) -> String {
        self.mnemonic.into_phrase()
    }

    fn from_phrase(phrase: &str) -> Result<Self> {
        let mnemonic = _Mnemonic::from_phrase(phrase, LANG)?;
        Ok(Self { mnemonic })
    }

    fn to_seed(&self, password: &str) -> HDSeed {
        HDSeed::new(Seed::from_bytes(
            _Seed::new(&self.mnemonic, password).as_bytes().to_vec(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_generated_seed_is(expected_seed: &str, phrase: &str, password: &str) {
        assert_eq!(
            expected_seed,
            hex::encode(
                Bip39Mnemonic::from_phrase(phrase)
                    .unwrap()
                    .to_seed(password)
                    .master_seed()
                    .to_bytes()
            )
        );
    }

    #[test]
    fn twelve_words_without_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_seed_is(
            "0b562acf3e3376b34a9c729bcfb265463b1f942d4b30e36fb832714c97336ad84baf003ee4fb6268779bcfb38f559f4eb95e59eb83a8746b0de356d8c285db17",
            "lunch blanket cruise chair question good market allow blue celery little void",
            "");
    }

    #[test]
    fn twelve_words_with_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_seed_is(
            "1e54c14991feab5a1cebbc58759a574298ac1002d4b38fc526f6ab0f5aa8967e9540e3922b983575e8eded188d7e392349d113659e191680f24c2ac13d0fa7e9",
            "lunch blanket cruise chair question good market allow blue celery little void",
            "my password");
    }

    #[test]
    fn fifteen_words_without_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_seed_is(
            "d0a56ba0f3725d43d457fd6998194c7ea4b8e0857588bc3e7ceee115ce4db70b877f0c52dbca8ebc190c18c88da9fdfe4d70471cac1b8b8011e9defe0e6d2e6f",
            "mirror distance build unaware current concert link chapter resemble tuition main rent echo drum dolphin",
            "");
    }

    #[test]
    fn fifteen_words_with_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_seed_is(
            "7cf09161993ba6c54e46212fd2aab9970d65e3c2236794b177bde0f3c932dfee123753cfb5891542465851d8d9dd15027333c148106616ac8dc0a1917633a3e2",
            "mirror distance build unaware current concert link chapter resemble tuition main rent echo drum dolphin",
            "my password");
    }

    #[test]
    fn eighteen_words_without_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_seed_is(
            "ef7686bf8147cd55933c3f44da14e8651134b26b2fdd2b1c7e90c2fd65e57d88e7bc1f23ffd32b48a359d8877c0e57641f7792f707f6ee5ce49eec28544af868",
            "blush section drift canoe reform friend rose cherry assume supreme home hub goat arena jazz absurd emotion hidden",
            "");
    }

    #[test]
    fn eighteen_words_with_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_seed_is(
            "b3a90e2b0c5f52416712a32106057f3ac8337c35d1df8cee1d88d9f4ff1411837d8fa19f87eb1887ab0dc3cab362e9dbf7dab981d3bedda9da721ed194700e38",
            "blush section drift canoe reform friend rose cherry assume supreme home hub goat arena jazz absurd emotion hidden",
            "my password");
    }

    #[test]
    fn twentyone_words_without_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_seed_is(
            "bdd875153c21427f0961c314fa9e12eeeb56fc897dc3c2cc6ef373b4acedcb3307b7246a24a8fb3cb05f4deab7915f93d5db4c26fccbf75a03d09983a5f4360d",
            "include disagree sentence junior gospel engage whip old boost scrap someone helmet list best afraid favorite gold antenna before peasant buffalo",
            "");
    }

    #[test]
    fn twentyone_words_with_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_seed_is(
            "3eea239974870a444037be1961d16ef1e358c9b4a18423a86d1f1a89c3c01648a2cacc762922f12931a585ceaa41100942afb3c76d8070ea4250f6ba7ec82011",
            "include disagree sentence junior gospel engage whip old boost scrap someone helmet list best afraid favorite gold antenna before peasant buffalo",
            "my password");
    }

    #[test]
    fn twentyfour_words_without_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_seed_is(
            "8012b1fcdd9275ba80e611a00bf8593b623dd690ecefe7f62868229666afa40d1497f3d8169d46dbbf32f023ee349548b7d4ac308cfeb5d3fcca5ee7af1a066d",
            "table car outdoor twist dutch auction monitor rude pumpkin very disease ability hope area metal brisk luggage tell ribbon profit various lake topic exist",
            "");
    }

    #[test]
    fn twentyfour_words_with_password() {
        // created with https://iancoleman.io/bip39/
        expect_generated_seed_is(
            "04c3fca05109eb0d188971e66ba949a4a4547b6c0eceddcb3e796e6ddb7d489826901932dbab5d6aa71421de1d119b4d472a92702e2642b2d9259d4766d84284",
            "table car outdoor twist dutch auction monitor rude pumpkin very disease ability hope area metal brisk luggage tell ribbon profit various lake topic exist",
            "my password");
    }

    #[test]
    fn generated_phrase_is_24_words() {
        let phrase = Bip39Mnemonic::generate().into_phrase();
        assert_eq!(23, phrase.chars().filter(|a| *a == ' ').count());
    }
}
