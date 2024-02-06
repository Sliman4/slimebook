# NFTs

NFTs are tokens that can not be divided into smaller units, are unique and not
interchangeable. They can represent ownership of digital assets like art, music,
but can also be used as tickets to real-life events, access keys, and more. Here
are some properties of it:

- It's unique and only one person can own it at a time
- It can be freely transferred to someone else
- It's not divisible, you can't use half of it
- Usually NFTs are part of a collection of similar NFTs, but each NFT is unique
- Anyone can create an NFT, or download a JPEG and call it an NFT, but anyone can
  see who created it and decide if it's worth anything
- Usually they have an image that represents them and can be viewed on a marketplace,
  description, traits. For example, "green hair" can be a trait of an NFT, if the
  NFT collection contains multiple NFT characters with different hair colors. But
  it's not a requirement, some NFTs don't have an image at all

They can have more applications than simply being an image, here are some examples:

- Some projects issue NFTs that give access to beta versions of their games,
  airdrops of their tokens
- There are blockchain games that use NFTs as "items" that can be bought, sold,
  and used in the game
- Proof of ownership of a physical object, like a car or a house (as long as it's
  legally recognized, of course, which is not common in most countries yet)
- You can wrap your crypto account into an NFT and sell it, so the buyer will get
  access to your account, [account ID](../lvl4/account-ids.md), and all the assets
  in it
- A musician can sell an NFT of a song, and the buyer will get royalties from
  the song's sales
- NFTs can be used as tickets to events, and the event organizers can control who
  can enter by checking the NFTs

One more example would be a ticket to a flight, let's analyze its properties:

- It's unique, the same ticket doesn't exist twice
- You can (depending on the airline's policy) give or sell it to someone else, or
  buy it from someone else
- It's not divisible, you can't use half of it to fly half the distance
- It's not interchangeable, you can't use it to fly with a different airline
- There are many similar tickets, but they are not completely the same. For example,
  they can have different prices, conditions, seats, etc.
- Anyone can "print a ticket" and sell it, but only the airline can issue a valid
  ticket, and you can't use a fake ticket to fly

# How to get NFTs

You can buy them on a marketplace, or mint them for free. Here are the most popular
NFT marketplaces on NEAR:

- [Paras](../lvl2/trade-nfts-paras.md)
- [Mintbase](../lvl4/mintbase.md)

There is also an NFT aggregator [TradePort](../lvl4/tradeport.md)

## Free mints

Minting is a process of creating an NFT. It's usually done by the creator of the NFT,
but sometimes the creator allows anyone to mint their NFTs. If the creator allows
anyone to mint their NFTs for free, it's called a "free mint".

Sometimes creators give away their NFTs for free, either to anyone or with some
conditions. You can find them on Twitter, Discord, and other social media. A good
place to start would be the [Slime Community Discord](https://discord.gg/A5Uh4hhauh)
server. As much as I'd like to have a list of all free mints here, I wouldn't be able
to keep it up to date, so you should look for them yourself.

One of the main platforms that provide free mints is [Shard dog](../lvl5/shard-dog.md),
and there's an NFT that is always given away for free for a subscription to a newsletter --
[NearWeek NFT](https://subscribe.nearweek.com/). By the way, [NearWeek](../lvl2/nearweek.md)
is also a good source of information about the NEAR ecosystem.

Usually unlimited freemints like NearWeek NFT are not worth anything, but you might still
want to mint them for fun or for memories of some events.

When you mint an NFT, you may be charged a small [storage fee](../lvl4/storage.md),
usually between 0.01 and 0.05 near, keep that in mind. But your wallet will warn you
about this, anyway.

## NFT analysis

Back in 2021, people were buying useless NFTs for a lot of money, they were just
expensive pictures. Now that the market is more mature, popular NFTs usually have
a clear use case. If you're looking to buy an NFT that will do something for you
(give access to airdrops, increase in price, etc.), you should do a thorough
research and not base your decision on the price and the picture alone. Of course,
if you want to buy an NFT from your favorite digital artist just to support them,
that's a different story.

Some good indicators:

- Trading activity on the marketplace. If the last NFT purchase was 2 weeks ago,
  it's probably not liquid and people are not interested in it
- The creator's reputation. If the creator is a well-known person in the NFT space,
  a well-known artist or company, it's more likely that the NFT will have some value
- The NFT's utility. If it gives you access to something, it's more likely to be
  valuable than if it's just a picture
- The NFT's supply. If there are 1000 of the same NFT or just 10, it's more likely
  that the 10 will be more valuable, because they are more rare
- Community. If there's an active community around the NFT, it's more likely that
  the NFT will have some value, because the community will support it

Some flags that indicate that the NFT needs more research:

- There are 10000 NFTs, but only 30 of them are listed on marketplaces. They could
  all be owned by the creator (red flag), or in [NFT staking](../lvl5/nft-staking.md)
  contracts (green flag), or [burned](../lvl3/burning.md) (green flag), or no one
  cares about them and people gave up trying to sell them (red flag), or something else
- The NFT creator claims that the NFT will give you some benefits, but there's
  no information about it or it's explained in technical words that you can't
  understand. Usually, good creators can explain things they're building in simple
  words, and bad creators use a lot of technical words to confuse you into thinking
  "it's too complicated for me to understand, but looks like a smart investment".
  In these cases, you should ask for help from someone who understands the topic and
  who you trust, or just skip the NFT
- The NFT creator is anonymous. It's not a red flag by itself, but it's a good
  reason to do more research. Try to assess the ability of the creator to deliver
  what they promise, and the likelihood of them [running away with your money](../lvl3/scams.md#exit-scam)
- The NFT provides huge benefit that seem to be impossible to deliver. For example,
  100% APY in a stablecoin. You need to analyze the business model, where the
  benefits come from, and if it's [sustainable](../lvl3/scams.md#ponzi-scheme). If
  you can't understand it, it's a huge red flag
- The NFT is a [meme](../lvl3/memecoins.md). There's a 0.5% chance of doing a 100x,
  and a 99.5% chance of losing all your money. With experience, you can start to tell
  the difference between a good meme and a bad meme, but that's not something I can
  teach you in the book because I have no idea how people do it, lol

## Creating your own NFT

If you want to create your own NFT, there are 2 main ways to do it:
1. Use a platform that allows you to create NFTs without coding
2. Write a smart contract of your NFT

Most people use the first way, because it's easier and more accessible. The second
way provides more control over the NFT, but it's not needed for most use cases. Here
are some popular platforms for creating NFTs:
- [Paras](../lvl2/trade-nfts-paras.md) - a marketplace that allows you to mint NFTs
- [Mintbase](../lvl4/mintbase.md) - a platform that allows you to mint NFTs and create
  your own marketplace
- [Enleap](https://enleap.app/) and [Bodega](https://bodega.land/) are NFT launchpads,
  but at the time of writing, Enleap doesn't work, and Bodega is only available for
  whitelisted projects with no public information. If you want your project to be
  on a launchpad, I recommend contacting them directly to discuss the terms
