# Crypto Wallet Generator

[![smessmer](https://circleci.com/gh/smessmer/crypto-wallet-gen/tree/master.svg?style=svg)](https://app.circleci.com/pipelines/github/smessmer/crypto-wallet-gen)

This is a utility to generate seed phrases and to generate crypto currency wallets from a seed phrase.
This way, you only need to remember one seed phrase and can generate wallets from it for multiple currencies.
A password can be added in the generation step so that you need both the seed phrase and the password to generate the wallets and access your funds.
We support both [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) and [scrypt](https://en.wikipedia.org/wiki/Scrypt) for generating the keys from the mnemonic (see details further below) and use [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)/[BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki) for derivation. Keys can be derived for Bitcoin (BTC), Ethereum (ETH) and Monero (XMR) wallets at the moment, further coins could be added relatively easily.

Generating a wallet from a seed phrase is a good way to secure your funds. You can, for example, print out the seed phrase (or etch it into metal cards for extra durability)
and store it offline. With this seed phrase (and the chosen password, if any), you can always restore access to your funds if the hard drive with your
crypto money happens to die. Or you carry it with you to get access to your funds from somewhere else.

## Usage

#### 1. Generate seed phrase and bitcoin wallets for it

```
$ crypto-wallet-gen -c BTC
Password: 
Repeat Password:
Mnemonic: select smart all afford joke rate soul abuse cover text receive height glimpse depend absent barely provide dilemma depend humor toy sing sock client
Password: [omitted from output]
--------------------------------------------------------------------------------------
BIP44 Derivation Path: m/44'/0'/0'
Private Key: xprv9ynrghM3dQW7K7v5r3Rh7x8AKuK3mU7rFNyqkKxrxqF7ff3cJmKTaciK4TMskUSoCuGgFLLrzyAYv6h1hd4d65t24oRRwzC2EBSXgP8mRUM
--------------------------------------------------------------------------------------
BIP44 Derivation Path: m/44'/0'/1'
Private Key: xprv9ynrghM3dQW7Li9ELovUZCZQ38Li2BhkKHw2WB8F8GEGMgGmWqrzp4Scaa5cXZDuxb6bZB6V4mHQSrJk8ZNpTNegb897iSy9W7bj45mnFhd
--------------------------------------------------------------------------------------
BIP44 Derivation Path: m/44'/0'/2'
Private Key: xprv9ynrghM3dQW7PYTEyJBKzFB5cjRFJ7ZsN5cy2wJjfZa5xQLQbGWEbqiWEBvp9kSty7HBLKqTrMbBMcQGg627ouARF3baK3uBA1Wf9Xzzi1r
```

The "mnemonic" is the seed phrase you need to remember or print.
The Private Keys can be entered to import the bitcoin wallets into your favourite bitcoin client.

#### 2. Generate a Monero wallet with the same seed phrase

```
$ crypto-wallet-gen -c XMR --from-mnemonic "select smart all afford joke rate soul abuse cover text receive height glimpse depend absent barely provide dilemma depend humor toy sing sock client"
Password: 
Repeat Password: 
Mnemonic: select smart all afford joke rate soul abuse cover text receive height glimpse depend absent barely provide dilemma depend humor toy sing sock client
Password: [omitted from output]
--------------------------------------------------------------------------------------
BIP44 Derivation Path: m/44'/128'/0'
Address: 46xZUoAc8dBVrYNfMLac5911Kveb2E4BXFTyp5m5sgyyhhan8rs6apQLLt6n3hLrK9QievKMbKMG9ceEsF4gURf7Msqav8Y
Private View Key: 91e54a0953ae7bdb711a6ec9d143f468d80071e3d4cb88d8d558f65841943804
Private Spend Key: 2afb68bd942c91361448494e1e032bfd184951d27c7ea87d78c299dd9e824303
--------------------------------------------------------------------------------------
BIP44 Derivation Path: m/44'/128'/1'
Address: 472GfMruChbEdBrkxCQv8YHLDsQGSG6rDRA2eJgvZQY32crtuCn93yZjDjVCVh8tnrZn6XdeeMKxWcbtp1DN6rpvHsmdCge
Private View Key: d551cdccb896d2853b5157b9fdd5c008a02510418ef3a18adde88d420a1d9504
Private Spend Key: fb6b0298478e86c913cef87dd5fc571080ea0224759b231827103904d4f4b800
--------------------------------------------------------------------------------------
BIP44 Derivation Path: m/44'/128'/2'
Address: 47YThN2R4xwSZjpoxEyhFsYKrgsHUV2SbiuK91r3X14XMmN8XSSqndyX6j5M1hYx1GciTr6Rkiv1vBmzi64oX4UnDUtw6BP
Private View Key: 3e6e33d85bd192934ab83aeacf97a9c6be019f85df1a77fd2a7d6451e07a2e07
Private Spend Key: bc3bb943ee34f10aee67b362b73d71b0d6c4f3edb8217a8e053fbd7277b14907
```

That's it. The addresses, private spend keys and private view keys can be used to import the wallets into the Monero client.

Now say you loose access to your Bitcoin or Monero wallet, using the mnemonic and step 2 above, you can always recover the Monero wallet again, and similarly you can recover your bitcoin wallet:

```
$ crypto-wallet-gen -c BTC --from-mnemonic "select smart all afford joke rate soul abuse cover text receive height glimpse depend absent barely provide dilemma depend humor toy sing sock client"
Password: 
Repeat Password: 
Mnemonic: select smart all afford joke rate soul abuse cover text receive height glimpse depend absent barely provide dilemma depend humor toy sing sock client
Password: [omitted from output]
--------------------------------------------------------------------------------------
BIP44 Derivation Path: m/44'/0'/0'
Private Key: xprv9ynrghM3dQW7K7v5r3Rh7x8AKuK3mU7rFNyqkKxrxqF7ff3cJmKTaciK4TMskUSoCuGgFLLrzyAYv6h1hd4d65t24oRRwzC2EBSXgP8mRUM
--------------------------------------------------------------------------------------
BIP44 Derivation Path: m/44'/0'/1'
Private Key: xprv9ynrghM3dQW7Li9ELovUZCZQ38Li2BhkKHw2WB8F8GEGMgGmWqrzp4Scaa5cXZDuxb6bZB6V4mHQSrJk8ZNpTNegb897iSy9W7bj45mnFhd
--------------------------------------------------------------------------------------
BIP44 Derivation Path: m/44'/0'/2'
Private Key: xprv9ynrghM3dQW7PYTEyJBKzFB5cjRFJ7ZsN5cy2wJjfZa5xQLQbGWEbqiWEBvp9kSty7HBLKqTrMbBMcQGg627ouARF3baK3uBA1Wf9Xzzi1r
```

All wallets recovered.

## Installation

#### 1. Install cargo (package manager for the rust programming language)

You can use [this one-step install command](https://www.rust-lang.org/tools/install).

You might have to call this afterwards, or alternatively just restart your bash session:
```
$ source $HOME/.cargo/env
```

Also make sure, you have openssl and a linker installed, for example by running the following:
```
$ sudo apt install libssl-dev pkg-config gcc
```

#### 2. Install crypto-wallet-gen
```
$ cargo install crypto-wallet-gen
```

## How keys are derived

This tool uses [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) for the mnemonic and [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)/[BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki) derivation from your seed phrase and password with the derivation path `m/44'/{coin}'/{address}'`.
That is, for bitcoin with address `0` (which is the default) we use `m/44'/0'/0'` and for monero `m/44'/128'/0'`.
For bitcoin, the derived key can be directly used as a bitcoin wallet. If such a key is imported into a bitcoin client like electrum, electrum derives `m/{change}/{index}` from the key it is given, so the full derivation path will match the BIP44 scheme of `m/44'/{coin}'/{address}'/{change}/{index}`.
For monero, we follow the algorithm described [here](https://github.com/libbitcoin/libbitcoin-system/wiki/Altcoin-Version-Mappings#10-monero-xmr-bip-3944-technology-examples), which means we interpret the private key part of the derived BIP32 extended key as a monero private key.

The example from that site is one of the integration tests in this repository. Also some examples generated at https://iancoleman.io/bip39/ .

### Scrypt derivation

There is an optional `--scrypt` parameter that replaces the [PBKDF2](https://en.wikipedia.org/wiki/PBKDF2) hash function of [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) with [scrypt](https://en.wikipedia.org/wiki/Scrypt).
This has three effects:

1. Somebody knowing your mnemonic but not the password who is trying to brute force the password will have a significantly harder time.
2. Generating a key from your mnemonic isn't instant anymore, it now takes several seconds (or minutes, depending on your hardware).
3. You're leaving BIP standards territory, there is no BIP standard for this. You cannot switch to a different tool and will be dependent on having this tool available when you want to generate keys from your mnemonic. Better keep a copy of the source code around just to be safe.
