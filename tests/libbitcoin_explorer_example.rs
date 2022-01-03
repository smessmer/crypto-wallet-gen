use crypto_wallet_gen::{
    Bip39Mnemonic, Bip44DerivationPath, BitcoinWallet, CoinType, EthereumWallet, Mnemonic,
    MnemonicFactory, MoneroWallet, Wallet,
};

#[test]
fn xmr_example_without_password() {
    // Example taken from https://github.com/libbitcoin/libbitcoin-system/wiki/Altcoin-Version-Mappings#10-monero-xmr-bip-3944-technology-examples
    let seed = Bip39Mnemonic::from_phrase(
        "radar blur cabbage chef fix engine embark joy scheme fiction master release",
    )
    .unwrap()
    .to_private_key("")
    .unwrap();
    let derived = seed
        .derive(&Bip44DerivationPath {
            coin_type: CoinType::XMR,
            account: 0,
            change: None,
            address_index: None,
        })
        .unwrap();
    assert_eq!(
        "e62551cad9fe0f05d7c84cf6a0ef7e8fc0534c2694279fc6e46d38f21a3f6ed3",
        hex::encode(derived.key_part().to_bytes()),
    );
    let wallet = MoneroWallet::from_hd_key(derived).unwrap();
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
        .to_private_key("")
        .unwrap();
    let derived = seed
        .derive(&Bip44DerivationPath {
            coin_type: CoinType::BTC,
            account: 0,
            change: Some(0),
            address_index: Some(0),
        })
        .unwrap();
    let wallet = BitcoinWallet::from_hd_key(derived).unwrap();
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
        .to_private_key("")
        .unwrap();
    let derived = seed
        .derive(&Bip44DerivationPath {
            coin_type: CoinType::BTC,
            account: 3,
            change: Some(1),
            address_index: Some(15),
        })
        .unwrap();
    let wallet = BitcoinWallet::from_hd_key(derived).unwrap();
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
        .to_private_key("My Password")
        .unwrap();
    let derived = seed
        .derive(&Bip44DerivationPath {
            coin_type: CoinType::BTC,
            account: 3,
            change: Some(1),
            address_index: Some(15),
        })
        .unwrap();
    let wallet = BitcoinWallet::from_hd_key(derived).unwrap();
    assert_eq!(
        "xprvA3mJpHT2oXZVZ7npWtcsonzQV4BuHQsmoWFPN1VQ3f2UVp34ZjnDziay8bwbLgxHuhvj2tqs3H4rbiZ7eESN3PUQEDcu2GmJKVoKSCKpBii",
        wallet.private_key(),
    );
}

#[test]
fn eth_example_without_password() {
    // Generated at https://myetherwallet.com
    let seed = Bip39Mnemonic::from_phrase(
        "tray busy leopard image soon twelve solar transfer donate inhale error chaos",
    )
    .unwrap()
    .to_private_key("")
    .unwrap();
    let derived = seed
        .derive(&Bip44DerivationPath {
            coin_type: CoinType::ETH,
            account: 0,
            change: Some(0),
            address_index: Some(0),
        })
        .unwrap();
    let wallet = EthereumWallet::from_hd_key(derived).unwrap();
    assert_eq!(
        "0x4d5475bED2Ce80fAaF21A2a773b63B7f5cB721db",
        wallet.address().unwrap(),
    );
}

#[test]
fn eth_example_with_password() {
    // Generated at https://myetherwallet.com
    let seed = Bip39Mnemonic::from_phrase(
        "tray busy leopard image soon twelve solar transfer donate inhale error chaos",
    )
    .unwrap()
    .to_private_key("My Password")
    .unwrap();
    let derived = seed
        .derive(&Bip44DerivationPath {
            coin_type: CoinType::ETH,
            account: 0,
            change: Some(0),
            address_index: Some(2),
        })
        .unwrap();
    let wallet = EthereumWallet::from_hd_key(derived).unwrap();
    assert_eq!(
        "0x169e507D6AB1c4Ab7840EB0A3C72cf5DbE85fadf",
        wallet.address().unwrap(),
    );
}

#[test]
fn eth_example_with_longer_mnemonic() {
    // This is an example generated with our own code, so it's a regression test making sure we keep generating the same keys
    let seed = Bip39Mnemonic::from_phrase(
        "fat label impose baby punch black oven wife gasp above eight fun canvas ready laundry impact blue inflict hawk supply guitar patrol cheap hard",
    )
    .unwrap()
    .to_private_key("My Password")
    .unwrap();
    let derived = seed
        .derive(&Bip44DerivationPath {
            coin_type: CoinType::ETH,
            account: 0,
            change: Some(0),
            address_index: Some(0),
        })
        .unwrap();
    let wallet = EthereumWallet::from_hd_key(derived).unwrap();
    assert_eq!(
        "0xE9F0681659503D5634AFa654CED1AeeE88A10272",
        wallet.address().unwrap(),
    );
    assert_eq!(
        "fa98c0ee1a7fc851883b098c72e0ab3c7276bdc327048f7ebec5658427de75f4",
        wallet.private_key(),
    );
    assert_eq!(
        "04f417d179b6d6ca7d90ed175bc74ead3a6c4266115c4f4069860b817ffb907b9b8dd806c8fe5c85ce1213cbd97d8d981dd01fe19f812df3690070db1d68b1e5",
        wallet.public_key(),
    )
}
