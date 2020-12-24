use crypto_keyderive::{
    derive_hd_wallet, mnemonic_to_seed, seed_to_wallet, Bip44DerivationPath, CoinType,
    MoneroWallet, Seed,
};

#[test]
fn libbitcoin_explorer_example_without_password() {
    // Example taken from https://github.com/libbitcoin/libbitcoin-system/wiki/Altcoin-Version-Mappings#10-monero-xmr-bip-3944-technology-examples
    let seed = mnemonic_to_seed(
        "radar blur cabbage chef fix engine embark joy scheme fiction master release",
        "",
    )
    .unwrap();
    let derived = derive_hd_wallet(
        seed,
        Bip44DerivationPath {
            coin_type: CoinType::XMR,
            account: 0,
            change: None,
            address_index: None,
        },
    )
    .unwrap();
    assert_eq!(
        "e62551cad9fe0f05d7c84cf6a0ef7e8fc0534c2694279fc6e46d38f21a3f6ed3",
        hex::encode(derived.as_bytes())
    );
    let wallet = seed_to_wallet(derived).unwrap();
    assert_eq!(
        "dd62d51183f6208cf4d1b9af523f2c80bf534c2694279fc6e46d38f21a3f6e03",
        wallet.private_spend_key()
    );
    assert_eq!(
        "deb53426c8ea9bc20581d0a9489e5b71df16219008c45e7747db98c42d7cf522",
        wallet.public_spend_key()
    );
    assert_eq!("4A4cAKxSbirZTFbkK5LwoYL3hLkVxkT8yLxAz8KCxAT66naEG4pYY9B6Q43zdao1oE3D3mzodbggzNz9t9tGvE8N3jVnu3A", wallet.address());
}
