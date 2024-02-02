# Account names
In NEAR, account names (account IDs) are human-readable names that are unique
across the entire network. They are used to identify accounts and can be used
to receive tokens, deploy contracts, and more. Because of that, you can see
in your wallet that are trying to send tokens to `slimedragon.near`, or
interact with [v2.ref-finance.near](../lvl2/exchanging-tokens-ref.md), which are
legitimate addresses, the same way as you can do a search and see that you're
searching on `google.com`, not a fake copy.

## Valid account names
There are 2 types of account names: implicit and named. Implicit account
names are the ones that consist of 64 seemingly random characters. If you don't
want to be ashamed of your address when someone asks you for an address, you
shouldn't use it. Named accounts are the ones that are human-readable and
can be used to receive tokens, deploy contracts, and more. Examples:
- `slimedragon.near`
- `root.near`
- `sub.slimedragon.near`
- `slimedrgn.tg`

Named account IDs can contain alphanumeric symbols and hyphens, and must be
between 2 and 64 characters long. They can't start or end with a hyphen, and
can't have two or more consecutive hyphens.

## Top-level names

Most commonly you'll encounter wallets ending with `.near`, but there are other
top-level names:
- `.tg` - Can be created in [Telegram NEAR Wallet](../lvl1/wallets/telegram-near-wallet.md)
- `.sweat` - Cannot be created by users, used by [Sweat Economy](../projects/sweat-economy.md)'s smart contracts
- `.kaiching` - Cannot be created by users, used by [KAIKAI](../projects/kaikai.md)'s smart contracts
- `.vrtx` - Used by Vortex Gaming, a gaming media community in Korea
- `.social` - Currently not being used, most likely related to [near social](../lvl3/near-social.md)
- `.calimero` - Currently not being used, owned by Calimero, an enterprise blockchain company with various infrastructure products
- `.aurora` - Cannot be created by users, used by [Aurora](../lvl5/aurora.md)

Top-level (without `.something`) account IDs with lengths >= 32 characters can be
created by anyone, though they're used very often. For example, `this-account-is-owned-by-a-green-slime`
is a valid account ID that can be created by anyone. Top-level accounts with length less
than 32 characters can only be created by [registrar](https://nearblocks.io/address/registrar), a system account. In the future,
there may be auctions of top-level accounts, but now they are given manually by the team.

## Subaccounts
If you have an account with name `slimedragon.near`, you (and only you!) can create
an account called `something.slimedragon.near`. But if you give someone this account,
you no longer have access to it, even if you still have full control over `slimedragon.near`.
Also, `slimedragon.near` can't create `a.b.slimedragon.near` without first creating `b.slimedragon.near`.
Technically, `near` is also a top-level account, all `.near` accounts are its subaccounts,
`near` doesn't have access to `.near` accounts, and can't create `something.slimedragon.near`
without `slimedragon.near`'s permission, if that makes it easier to understand.
