# Getting started

Of course, the first thing you need to do is to create a NEAR account using a wallet.

There is no official NEAR Wallet[^1], but there are a lot of wallets
created by other teams. The most popular ones are:
- [Meteor Wallet](meteor-wallet.md) - web + chrome extension
- [HERE Wallet](here-wallet.md) - mobile
- [MyNearWallet](my-near-wallet.md) - web
- [NEAR Mobile Wallet](near-mobile-wallet.md) - mobile
- [Sender Wallet](sender-wallet.md) - mobile + chrome extension
- [Nightly Wallet](nightly-wallet.md) - chrome extension + firefox extension
- [Telegram NEAR Wallet](telegram-near-wallet.md) - telegram app

There is [a full list of all wallets and their features](https://docs.google.com/spreadsheets/d/1Q9ZEeWpFHgcPthSCvzyiVcaKdBIcdS3r96v48OYsDBM/edit),
but I don't recommend using lesser-known wallets, especially if you're new to
the ecosystem, it'll just waste your time and (most likely) not bring any
benefits.

While I aim to keep this book objective, I know you probably don't want to
read all these pages and compare it yourself, so here's what I recommend:
- If you're on a computer, use [Meteor Wallet](meteor-wallet.md).
- If you're on a phone, use [HERE Wallet](here-wallet.md).

## Connecting to dapps

Once you have a wallet, you can connect it to a dapp (decentralized app) and
start using it. You can find the most popular dapps in this book and https://www.discoverbos.org/projects,
though some of them might be dead or outdated.

When you click "Connect a wallet" on the dapp's website. it will ask you to
choose a wallet and then your wallet will ask you to confirm the connection.
When you connect your wallet to a dapp, you give it a permission to read your
account name, balance, and sometimes, to send transactions on your behalf. Don't
worry, it can't transfer tokens and NFTs without your confirmation, even if you
connected it to a scam dapp.

Usually, these connections also request a 0.25 NEAR allowance for transaction fees,
but it doesn't use your 0.25 NEAR right away, your NEAR stays in your account,
but the dapp *can* use some portion of it to pay for the transactions that it sends.
0.25 NEAR is the standard amount that dapps request, even if they don't need to do any
transactions. Your 0.25 NEAR is not frozen, you don't even need to have 0.25 NEAR to
connect, it's just an allowance to use *up to* that much. But every connection creates
a [key](../../lvl4/account-model/keys/index.md#function-call-access-key) on blockchain,
which costs about 0.00004 NEAR, and you need some NEAR for [storage](../../lvl4/account-model/storage.md)
of this key, so you can't connect to dapps if you don't have any NEAR at all. Logging out
deletes the key, which also costs about 0.00004 NEAR.

## Safety of your wallet

Most wallets will give you a seed phrase (pass phrase) when you create an
account. It's a list of (usually) 12 words that you should keep safe and never
share with anyone. Even the developers of the wallet. This phrase can give access
to your account to anyone who knows it, but it can also be used to restore your
account if you lose access to your wallet, so it's important to keep it safe and
secret. For the best practices, see [Where to save your seed phrase](../../lvl4/account-model/keys/where-to-save-seed-phrase.md).

[^1]: There *was* an official [NEAR Wallet](https://wallet.near.org/), but it was
deprecated in favor of the community wallets, and discontinued in January 2024.
MyNearWallet is the closest to the official wallet (a fork), but now it's developed by
a community team. But in my opinion, it's the worst user experience I've ever seen in
a crypto wallet, so I don't recommend using it.
