# Stablecoins

Stablecoins are cryptocurrencies that are pegged to a stable asset, such as fiat currency,
gold, or other commodities. They are designed to remove the volatility of the cryptocurrency,
so that you don't have to worry about the price of your cryptocurrency changing. This makes
them a great option for people who want to use cryptocurrency for everyday transactions, but
not for investment purposes. The most popular stablecoins are Tether (USDT) and USD Coin (USDC),
which represent the US Dollar. If you receive 10 USDC, you can be sure that it will always be
worth $10.

## How do stablecoins work?

There are 3 types of stablecoins:

- **Fiat-collateralized stablecoins**: These are backed by a reserve of fiat currency, such
  as the US Dollar. For example, Tether (USDT) and USD Coin (USDC) are backed by a 1:1
  reserve of US Dollars and other very stable assets, such as US Treasury bonds. In case
  a lot of people want to redeem their stablecoins for fiat currency, the issuer (Tether
  or Circle, in the case of USDC) will have the necessary funds to redeem them and keep
  the price stable. Do they really have the necessary funds? This is a matter of trust
  and regulation, and it has been a controversial topic in the cryptocurrency space.
- **Crypto-collateralized stablecoins**: These are backed by a reserve of other
  cryptocurrencies. For example, DAI is backed by a reserve of Ether (ETH), [liquid staked](staking.md#liquid-staking)
  Ether, other stablecoins, real-world assets, and other cryptocurrencies.
  The price of DAI is kept stable through a system of smart contracts
  and over-collateralized debt positions. It means you can supply some amount of
  collateral (more than 100%), and create 1 DAI. Or redeem 1 DAI for $1 worth of some
  collateral. At the time of writing, the collaterization ratio is [almost 300%](https://daistats.com/#/overview),
  meaning that for every 1 DAI there is $3 worth of other assets. If the price of DAI
  goes up, new DAI is created and sold to bring the price down. If the price of DAI goes
  down, DAI is bought and burned to bring the price up. This might be a bit more complex,
  but it doesn't rely on trust in a centralized issuer. What if the price of the collateral
  goes down so fast that the system can't sell it fast enough? This is a good question.
- **Algorithmic stablecoins**: These are not backed by any reserve, but they use algorithms
  to keep the price stable. There are different mechanisms to achieve this, so it's hard
  to generalize. This is a very experimental field, and by far the riskiest one. One of the
  most popular algorithmic stablecoins
  was [TerraUSD](https://en.wikipedia.org/wiki/Cryptocurrency_bubble#Collapse_of_Terra-Luna),
  anyone who knew how it worked, knew the risks, but many lost all their money. I recommend
  you to read whitepapers if you are a technical person and you are interested in this
  topic, but if you don't understand how they work and don't want to expose yourself to the
  unnecessary risk, I recommend you to stay away from them.

## Stablecoins on NEAR

On NEAR Protocol, the most widely used stablecoins are:

- USDT - a fiat-collateralized stablecoin, you can use it like any other [FT](../lvl1/fts.md).
- USDC - a fiat-collateralized stablecoin, you can use it like any other [FT](../lvl1/fts.md).
- USN - a half-crypto-collateralized half-algorithmic stablecoin. It launched in a very bad timing,
  right before TerraUSD's collapse, and although the algorithm is completely different, it
  has [shut down](https://near.org/blog/statement-in-full-near-foundation-to-fund-usn-protection-programme) in October 2022. While now it's not very widely used nowadays, some
  protocols still have trading pairs and liquidity pools for USN, so it's still worth mentioning here.
  I don't recommend to use it, though, it's dead and it's not coming back.

## What are USDT.e and USDC.e?

A few years ago, there was no USDT or USDC on NEAR, but many people wanted to have a
stablecoin or two in the NEAR ecosystem. So, they created a [bridged](rainbow-bridge.md)
version of USDT and USDC, called USDT.e and USDC.e. They are created by using the
Rainbow Bridge to lock the original USDT and USDC on Ethereum, and mint the equivalent
amount of USDT.e and USDC.e on NEAR. But now that there are "native" versions of USDT and
USDC on NEAR, issued directly by Tether and Circle, respectively, there is no need to have
USDT.e and USDC.e anymore. But it's not that easy to get rid of them, because there are
still a lot of people who use them, and a lot of trading pairs and liquidity pools that
will be hard to migrate. So, they are still around. It's important to note that USDT.e
and USDC.e are backed by the real USDT and USDC on Ethereum, but if the Rainbow Bridge
gets hacked (which happens quite frequently with other bridges), there could be negative
consequences for USDT.e and USDC.e prices. So, if you have the option to use native USDT
and USDC, I recommend you to use them instead. But Rainbow Bridge is a very secure bridge
compared to others, and there are some benefits to using the bridged versions of stablecoins -
if you [swap](exchanging-tokens-ref.md) a token to it, in some cases you may get a
slightly better price, so it's not a big deal if you use USDT.e and USDC.e.

## Contract addresses

- Native USDT: [`usdt.tether-token.near`](https://nearblocks.io/token/usdt.tether-token.near)
- Native USDC: [`17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1`](https://nearblocks.io/token/17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1)
- Bridged USDT: [`dac17f958d2ee523a2206206994597c13d831ec7.factory.bridge.near`](https://nearblocks.io/token/dac17f958d2ee523a2206206994597c13d831ec7.factory.bridge.near)
- Bridged USDC: [`a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.factory.bridge.near`](https://nearblocks.io/token/a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48.factory.bridge.near)
- USN: [`usn`](https://nearblocks.io/token/usn)
- Testnet USDT ([for developers](https://near-faucet.io/)): [`usdt.fakes.testnet`](https://testnet.nearblocks.io/address/usdt.fakes.testnet)
- Testnet USDC ([for developers](https://near-faucet.io/)): [`usdc.fakes.testnet`](https://testnet.nearblocks.io/address/usdc.fakes.testnet)
