# Crypto Wallet Generator

[![smessmer](https://circleci.com/gh/smessmer/crypto-wallet-gen/tree/master.svg?style=svg)](https://app.circleci.com/pipelines/github/smessmer/crypto-wallet-gen)

This is a utility to generate seed phrases and to generate crypto currency wallets from a seed phrase.
This way, you only need to remember one seed phrase and can generate wallets from it for multiple currencies.
A password can be added in the generation step so that you need both the seed phrase and the password to generate the wallets and access your funds.
We support both [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) and [scrypt](https://en.wikipedia.org/wiki/Scrypt) for generating the keys from the mnemonic (see details further below) and use [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)/[BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki) for derivation.

Generating a wallet from a seed phrase is a good way to secure your funds. You can, for example, print out the seed phrase (or etch it into metal cards for extra durability)
and store it offline. With this seed phrase (and the chosen password, if any), you can always restore access to your funds if the hard drive with your
crypto money happens to die. Or you carry it with you to get access on your funds from somewhere else.

## Usage

#### 1. Generate seed phrase and a bitcoin wallet for it

```
$ crypto-wallet-gen -c BTC
Password: 
Repeat Password:
Mnemonic: border visit cupboard great address trumpet cash either castle rubber tape foil combine mix width burst crunch broccoli family resist fish build develop when
Password: [omitted]
WIF: KyFcTdhE77i1WLE6SsxiPkPSfst1w49t8PhveEWPu1zdjJmQ5V9t
```

The "mnemonic" is the seed phrase you need to remember or print.
The WIF can be entered to import the bitcoin wallet in your favourite bitcoin client.

#### 2. Generate a Monero wallet with the same seed phrase

```
$ crypto-wallet-gen -c XMR --from-mnemonic "border visit cupboard great address trumpet cash either castle rubber tape foil combine mix width burst crunch broccoli family resist fish build develop when"
Password: 
Repeat Password: 
Mnemonic: border visit cupboard great address trumpet cash either castle rubber tape foil combine mix width burst crunch broccoli family resist fish build develop when
Password: [omitted]
Address: 43pFzcvF5SVWrcvSb3t6d85ZWfD2BW4oyGj1dmzR63JSHvzRnfhNKUdBJQxmFD8JCWNRBRJuH9p3LbrYiuq3CDnkTCWWvW7
Private Spend Key: a39ed91d4bd30080cc4e4a9e8a8ca6be3d7eb92648f7ee211fae8c2335440009
Private View Key: c654cd31f28a12a711b17a6b286c92ae7ab574c2992935a56aa5ac40bf3cff0d
```

That's it. The address, private spend key and private view key can be used to import the wallet into the Monero client.

Now say you loose access to your Bitcoin or Monero wallet, using the phrase and step 2 above, you can always recover the Monero wallet again, and similarly you can recover your bitcoin wallet:

```
$ crypto-wallet-gen -c BTC --from-mnemonic "border visit cupboard great address trumpet cash either castle rubber tape foil combine mix width burst crunch broccoli family resist fish build develop when"
Password: 
Repeat Password: 
Mnemonic: border visit cupboard great address trumpet cash either castle rubber tape foil combine mix width burst crunch broccoli family resist fish build develop when
Password: [omitted]
WIF: KyFcTdhE77i1WLE6SsxiPkPSfst1w49t8PhveEWPu1zdjJmQ5V9t
```

## Installation

#### 1. Install the rust programming language

You can use [this one-step install command](https://www.rust-lang.org/tools/install).

You might have to call this afterwards, or alternatively just restart your bash session:
```
$ source $HOME/.cargo/env
```

Also make sure, you have openssl and a linker installed, for example by running the following:
```
$ sudo apt install libssl-dev pkg-config gcc
```

#### 2. Check out the crypto-wallet-gen repository and install it
```
$ git clone https://github.com/smessmer/crypto-wallet-gen
$ cd crypto-wallet-gen
$ cargo install --path .
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
