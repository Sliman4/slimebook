# Storage fee

There's no such thing as free storage. When you store something on the NEAR
blockchain, you pay for it. The price is low (1 NEAR for 100kb), but when
millions of users store their data, it becomes a significant amount of money.
Not to mention, [sybil attacks](https://docs.near.org/develop/contracts/security/sybil)
can quickly drain the developer's balance. That's why **users** have to pay for their
storage. The good thing is, as soon as you delete the data, your money is fully refunded.

An average NEAR account costs about 0.06 NEAR to maintain, this amount is usually
deducted from your first deposit of NEAR. After that, the most common thing that
you'll store is your tokens - FTs and NFTs. Usually the fee is between 0.01 and 0.1
NEAR for each token, and it's paid when you receive your first token. For [FT](../../lvl1/fts.md)s
it's paid once because it's just an increasing number that takes the same amount of bytes,
and for [NFT](../../lvl1/nfts.md)s it's paid for each token because they're all unique.

But actually, the storage fee is not paid, it's frozen on the account that stores data.
If you use an [FT](../../lvl1/fts.md), the data is stored in the FT contract, not in your
account, so the fee is paid to the FT contract and it's frozen there. If an account
doesn't have enough NEAR to pay for storage, the transaction fails.

[Smart contract](../../lvl3/smart-contracts.md) code also requires storage, the bigger the
contract is, the more NEAR it requires to have. The transaction data, that is available
on the [explorer](../../lvl3/nearblocks.md), doesn't require storage, since it's not stored
on the blockchain, and dismissed[^1] right after the transaction ends.

[^1]: The full transaction history, including the transaction data, is stored in the
      [archival nodes](../../lvl6/node-types.md) and [indexers](../../lvl6/indexers.md).
