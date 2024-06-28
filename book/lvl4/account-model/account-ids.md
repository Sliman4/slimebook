# Account names
In NEAR, account names (account IDs) are human-readable names that are unique
across the entire network. They are used to identify accounts and can be used
to receive tokens, deploy contracts, and more. Because of that, you can see
in your wallet that are trying to send tokens to `slimedragon.near`, or
interact with [v2.ref-finance.near](../../lvl2/exchanging-tokens-ref.md), which are
legitimate addresses, the same way as you can do a search and see that you're
searching on `google.com`, not a fake copy.

## Valid account names
There are 2 types of account names: implicit and named. Implicit account
names are the ones that consist of 64 seemingly random characters. Example: [998765b2120bf0ca10a9242343fdbda6612a48b279d69c9e4d99dbf5adda7d93](https://nearblocks.io/address/998765b2120bf0ca10a9242343fdbda6612a48b279d69c9e4d99dbf5adda7d93).
If you don't want to be ashamed of your address when someone asks you for it, you
shouldn't use it. Instead, use named accounts. Named accounts are human-readable.
Examples:
- `slimedragon.near`
- `root.near`
- `sub.slimedragon.near`
- `slimedrgn.tg`

Named account IDs can contain alphanumeric symbols, hyphens, underscores, and must be
between 2 and 64 characters long. They can't start or end with a hyphen, and
can't have two or more consecutive hyphens.

## Implicit accounts

Implicit accounts are accounts that contain 64 characters 0-9 and a-f. They cannot be created
like normal named accounts, they always exist, but are initialized only when they receive
some NEAR. By default, they have a [full key](keys/index.md#full-access-key) with public key
the same as its address (but encoded in base58), and no other keys, so it's impossible
to create an account with a predetermined implicit account ID.

## Top-level names

Most commonly you'll encounter wallets ending with `.near`, but there are other
top-level names, like `.tg` (can be created in [HOT Wallet](https://t.me/herewalletbot/app)),
`.sweat` (cannot be created by users, used by [Sweat Economy](../../projects/sweat-economy.md)'s
[smart contracts](../../lvl3/smart-contracts.md)), `.kaiching`, `.vrtx`, `.aurora`, and
so on, but most of them are used internally and cannot be created by users.

Top-level (without `.something`) account IDs with lengths >= 32 characters could be
created by anyone, but they were used very often, and now it's not possible. For example, [this-account-is-owned-by-a-green-slime](https://nearblocks.io/address/this-account-is-owned-by-a-green-slime)
is a valid account ID that can be created by anyone. Top-level accounts can only be created by [registrar](https://nearblocks.io/address/registrar), a system account. In the future,
there may be auctions of top-level accounts, but now they are given manually by `registrar` owner, which is most likely controlled by Pagoda or Near Foundation.

## Subaccounts

If you have an account with name `slimedragon.near`, you (and only you!) can create
an account called `something.slimedragon.near`. But after you create this account, it's
a fully independent account.

Technically, `near` is also a top-level account, all `.near` accounts are its subaccounts,
but `near` doesn't have access to your `.near` account, and can't create `slimething.slimedragon.near`,
if that makes it easier to understand. For more information, visit [docs.near.org](https://docs.near.org/concepts/basics/accounts/account-id)
