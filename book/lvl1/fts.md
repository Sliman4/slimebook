# Fungible Tokens (FTs)

Fungible Tokens (FTs) are tokens that are interchangeable with each other. They
are identical in specification and value. For example, if you have 10 FTs, and someone
else has 10 FTs, you are both holding the same thing. The same way as 1 dollar equals
to another 1 dollar. They have the same properties as money:

- Can be split into smaller parts, 10 dollars = 10 * 1 dollar, 1 dollar = 100 cents
- Can be transferred to someone else
- Can be used to pay for goods and services, or as rewards for doing something

Examples of FTs:

- **$NEAR** is used to pay for [transaction fees](../lvl4/transactions.md#gas-fees),
  [storage](../lvl4/account-model/storage.md), can be earned by [staking](../lvl2/staking.md) and
  by developing an app that people use (part of transaction fees goes to the app owner
  and the rest is [burned](../lvl3/burning.md))
- **$USDC** and **$USDT** are [stablecoins](../lvl2/stablecoins.md), they always cost
  1 US dollar, and are used to transfer money between people and to pay for goods and
  services without worrying about the price volatility
- **$REF** is a token of the [Ref Finance](../lvl2/exchanging-tokens-ref.md) exchange,
  it's used to reward [liquidity providers](../lvl4/providing-liquidity-ref.md) (people who give money to the exchange)
  in REF tokens, and give REF token holders a share of the exchange's fees
- **$LiNEAR** is a token that represents [staked](../lvl2/staking.md#liquid-staking) NEAR,
  that preserves both the staking rewards and the ability to use your money for other
  purposes
- **$BLACKDRAGON** is a [memecoin](../lvl3/memecoins.md) that doesn't have a particular
  use case as of yet, but some people like it and buy it for fun

## How to get tokens

- **Buy** them on an [online exchange](../lvl2/exchanging-tokens-ref.md) or from someone
  who has them
- **Earn** them by doing something that the project rewards with tokens
- **Mint** them yourself, if the project allows it. "Minting" means creating new tokens
  and putting them into circulation. Some projects allow you to mint tokens for free,
  some require you to pay for it or to do something else in exchange for the tokens
- **Create** your own token. You can create a new token on NEAR, decide how many
  tokens there will be, what they will be called, and what they will be used for,
  add it to an exchange, and sell it to people, but you will have to convince people
  that your token is worth something

### More about "earn"

Every project has its own way of rewarding people with tokens. Here are some of the most
common ways projects reward people with tokens:

- **Staking** - you lock your tokens for a certain period of time and get rewards
  for doing so. If the token you want to earn has staking, you can find more details
  on the project's website because it's different for each project. From the list above,
  **NEAR** and **REF** have staking
- **Providing liquidity** - you give your tokens to an exchange, and in return you
  get a share of the exchange's fees. Every token that is listed on an exchange can
  be used to provide liquidity, but the amount of fees you get depends on the token
  trading volume
- **Liquidity mining** - In addition to exchange's fees, sometimes you can get
  additional tokens as an incentive, if the project wants to attract more liquidity.
  Usually, liquidity mining is temporary, since projects have a specific budget for
  the incentives, but new programs can be announced at any time
- **Airdrops** - some projects give away tokens for free to people who have a certain
  token, [NFT](nfts.md), to people who used their product in the past, or to
  people who are active in the NEAR ecosystem. Follow the project's social media
  and announcements to find out about future airdrops. Sometimes they are announced
  in advance, sometimes they are given to people who are already using the project,
  these are called "retroactive airdrops" or "retrodrops"
- **Bounties** - you do something that the project needs, and in return you get tokens.
  For example, you can write an article, a twitter thread, make a video, or help with
  the development of the project. Bounties are usually announced on the project's website
  or social media. There are platforms like [Heroes.build](https://heroes.build) that
  aggregate bounties from different projects
- **Referral programs** - you invite people to use the project, and in return you
  get a share of the project's fees or a one-time reward
- **Trading** - you buy tokens for a lower price and sell them for a higher price
  on an exchange. This is the riskiest way to earn tokens, because you can't predict
  the future price of the token, and you always pay fees to the exchange. But if you
  closely follow the project and the market, or know something that most people don't,
  you can increase your chances of earning tokens this way

## Fees

### Storage fee

When it's your first time interacting with a token, you might need to pay a [fee](../lvl4/account-model/storage.md) to
receive it. This is because the token balances are stored in a smart contract, and
the smart contract needs to store the information about your balance. This fee is
usually small, 0.01-0.10 NEAR for most coins.

If you're sending someone a token, and they don't have a balance of that token,
you may also be charged this fee. This is because the smart contract needs to store
the information about their balance anyway, or the [transaction](../lvl4/transactions.md)
will simply fail.

### Transfer fees

Some tokens, especially [memecoins](../lvl3/memecoins.md), have a fee for every
transaction. This makes it more expensive to trade them too often, and the fee
is usually either [burned](../lvl3/burning.md), given to the project's treasury,
or distributed to the token holders. You should always check the token's website
or the exchange's website to find out about the fees, because they are different
for each token and exchange.

## Creating your own token

If you want to create your own token, there are 2 main ways to do it:
1. Use a platform that allows you to create tokens without coding
2. Write a smart contract of your token

The first way is easier, but the second way gives you more control over the token. 
In this book, we will cover the first way, because it's easier and more accessible.
The most popular platform is Token Farm, but it's not usable anymore.

Someday, someone will create a simple website for deploying tokens and I'll add it
to this book.
