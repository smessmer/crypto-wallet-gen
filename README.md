Crypto Wallet Generator
-----------------------

This is a utility to generate seed phrases and to generate crypto currency wallets from a seed phrase.
This way, you only need to remember one seed phrase and can generate wallets from it for multiple currencies.
A password can be added in the generation step so that you need both the seed phrase and the password to generate the wallets and access your funds.

Generating a wallet from a seed phrase is a good way to secure your funds. You can, for example, print out the seed phrase (or etch it into metal cards for extra durability)
and store it offline. With this seed phrase (and the chosen password, if any), you can always restore access to your funds if the hard drive with your
crypto money happens to die. Or you carry it with you to get access on your funds from somewhere else.

Example
-------

1. Generate seed phrase and a bitcoin wallet for it

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

2. Generate a Monero wallet with the same seed phrase

```
$ crypto-wallet-gen -c XMR --from-mnemonic "border visit cupboard great address trumpet cash either castle rubber tape foil combine mix width burst crunch broccoli family resist fish build develop when"
Password: 
Repeat Password: 
Mnemonic: border visit cupboard great address trumpet cash either castle rubber tape foil combine mix width burst crunch broccoli family resist fish build develop when
Password: [omitted]
Address: 43pFzcvF5SVWrcvSb3t6d85ZWfD2BW4oyGj1dmzR63JSHvzRnfhNKUdBJQxmFD8JCWNRBRJuH9p3LbrYiuq3CDnkTCWWvW7
Private Key: a39ed91d4bd30080cc4e4a9e8a8ca6be3d7eb92648f7ee211fae8c2335440009
Public Key: 39d996da5d57f6b27e916d0f6492fb1b44045451a5a7d85e02757d560b582365
```

That's it. The address, private key and public key can be used to import the wallet into the Monero client.

Now say you loose access to your Bitcoin or Monero wallet, using the phrase and step 2 above, you can always recover the Monero wallet again, and similarly you can recover your bitcoin wallet:

```
$ crypto-wallet-gen -c BTC --from-mnemonic "border visit cupboard great address trumpet cash either castle rubber tape foil combine mix width burst crunch broccoli family resist fish build develop when"
Password: 
Repeat Password: 
Mnemonic: border visit cupboard great address trumpet cash either castle rubber tape foil combine mix width burst crunch broccoli family resist fish build develop when
Password: [omitted]
WIF: KyFcTdhE77i1WLE6SsxiPkPSfst1w49t8PhveEWPu1zdjJmQ5V9t
```


How keys are derived
---------------
This tool uses BIP32/BIP44 derivation from your seed phrase and password and the derivation path "m/44'/{coin}'/0'/0/0".
That is, for bitcoin we use "m/44'/0'/0'/0/0" and for monero we use "m/44'/128'/0'/0/0".
For bitcoin, the derived key can be directly used as a bitcoin wallet.
For monero, we follow the algorithm described in https://github.com/libbitcoin/libbitcoin-system/wiki/Altcoin-Version-Mappings#10-monero-xmr-bip-3944-technology-examples .

The example from that site is one of the integration tests in this repository. Also some examples generated at https://iancoleman.io/bip39/ .
