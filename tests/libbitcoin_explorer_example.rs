use crypto_wallet_gen::{
    Bip39Mnemonic, Bip44DerivationPath, BitcoinWallet, CoinType, Mnemonic, MoneroWallet, Wallet,
};

#[test]
fn xmr_example_without_password() {
    // Example taken from https://github.com/libbitcoin/libbitcoin-system/wiki/Altcoin-Version-Mappings#10-monero-xmr-bip-3944-technology-examples
    let seed = Bip39Mnemonic::from_phrase(
        "radar blur cabbage chef fix engine embark joy scheme fiction master release",
    )
    .unwrap()
    .to_seed("");
    let derived = seed
        .derive(Bip44DerivationPath {
            coin_type: CoinType::XMR,
            account: 0,
            change: None,
            address_index: None,
        })
        .unwrap();
    assert_eq!(
        "e62551cad9fe0f05d7c84cf6a0ef7e8fc0534c2694279fc6e46d38f21a3f6ed3",
        hex::encode(derived.private_key.to_bytes()),
    );
    let wallet = MoneroWallet::from_extended_key(derived).unwrap();
    assert_eq!(
        "dd62d51183f6208cf4d1b9af523f2c80bf534c2694279fc6e46d38f21a3f6e03",
        wallet.private_spend_key(),
    );
    assert_eq!(
        "deb53426c8ea9bc20581d0a9489e5b71df16219008c45e7747db98c42d7cf522",
        wallet.public_spend_key().unwrap(),
    );
    assert_eq!(
        "7838567e050aa2dc3964bca85c3a42d9cec5b77b3d8f055e2763641fdce53c07",
        wallet.private_view_key(),
    );
    assert_eq!("4A4cAKxSbirZTFbkK5LwoYL3hLkVxkT8yLxAz8KCxAT66naEG4pYY9B6Q43zdao1oE3D3mzodbggzNz9t9tGvE8N3jVnu3A", wallet.address().unwrap());
}

#[test]
fn btc_example_without_password() {
    // Generated at https://iancoleman.io/bip39/
    let seed = Bip39Mnemonic::from_phrase("sheriff cry practice silly depth still legal short mixture salad scan fever nephew solar hill correct birth wash banner mammal impose price kind spice")
        .unwrap()
        .to_seed("");
    let derived = seed
        .derive(Bip44DerivationPath {
            coin_type: CoinType::BTC,
            account: 0,
            change: Some(0),
            address_index: Some(0),
        })
        .unwrap();
    let wallet = BitcoinWallet::from_extended_key(derived).unwrap();
    assert_eq!(
        "xprvA3vaqsvkTobj2wczyNukcxwCFAFciX6XNJdQdAFgLiCYsssnLRb4FYC6qd6vaQWWL2EThqAhHHqxtWiK6ts9A8fY7Vizy6JEpsGjF8YMY2g",
        wallet.private_key(),
    );
}

#[test]
fn btc_example_subaddress_without_password() {
    // Generated at https://iancoleman.io/bip39/
    let seed = Bip39Mnemonic::from_phrase("sheriff cry practice silly depth still legal short mixture salad scan fever nephew solar hill correct birth wash banner mammal impose price kind spice")
        .unwrap()
        .to_seed("");
    let derived = seed
        .derive(Bip44DerivationPath {
            coin_type: CoinType::BTC,
            account: 3,
            change: Some(1),
            address_index: Some(15),
        })
        .unwrap();
    let wallet = BitcoinWallet::from_extended_key(derived).unwrap();
    assert_eq!(
        "xprvA37UqVh8aYoGwVuSMADMgJegXsYEe6q7jXGvCmxxcyLu5yaiphJXPDpKcvY2XRB4aeba3MU8R79U2fpTPggjHhmVRexLBWUEtsbhs4vEus2",
        wallet.private_key(),
    );
}

#[test]
fn btc_example_subaddress_with_password() {
    // Generated at https://iancoleman.io/bip39/
    let seed = Bip39Mnemonic::from_phrase("sheriff cry practice silly depth still legal short mixture salad scan fever nephew solar hill correct birth wash banner mammal impose price kind spice")
        .unwrap()
        .to_seed("My Password");
    let derived = seed
        .derive(Bip44DerivationPath {
            coin_type: CoinType::BTC,
            account: 3,
            change: Some(1),
            address_index: Some(15),
        })
        .unwrap();
    let wallet = BitcoinWallet::from_extended_key(derived).unwrap();
    assert_eq!(
        "xprvA3mJpHT2oXZVZ7npWtcsonzQV4BuHQsmoWFPN1VQ3f2UVp34ZjnDziay8bwbLgxHuhvj2tqs3H4rbiZ7eESN3PUQEDcu2GmJKVoKSCKpBii",
        wallet.private_key(),
    );
}
