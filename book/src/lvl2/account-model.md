# Account model

Accounts on NEAR have the following things:
- [Account ID](../lvl4/account-ids.md) like `slimedragon.near`
- [Access keys](./keys.md) like `ed25519:HDXaKmewwTBHp87V8tCZWqDkgMLbJ7Eb3jifMC38r2kw`
- NEAR balance [(token balances are stored in the tokens themselves!)](../lvl1/fts.md#storage-fee)
  Nothing too special here, just a number.
- Code. Every account can be a smart contract, and the code is stored in the account itself.
  But this book is not for people who want to write smart contracts, so I won't cover this part.
- Storage. Also used for smart contracts.