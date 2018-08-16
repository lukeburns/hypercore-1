# hypercore-red

**Experimental:** This is a fork of the WIP [Rust implementation](https://github.com/datrs/hypercore) of [hypercore](https://github.com/mafintosh/hypercore), a secure and distributed append-only log (with nice properties like sparse-replication). It replaces [ed25519](https://github.com/dalek-cryptography/ed25519-dalek) with an [experimental / insecure implementation](https://github.com/lukeburns/redschnorr) of the Schnorr signature scheme on the [Ristretto prime order group](https://doc.dalek.rs/curve25519_dalek/ristretto/), which has some nice mathematical properties. (Note: this implementation does not produce deterministic signatures like ed25519).

Working with Ristretto allows for some deterministic key derivation schemes that could be useful for decentralized identity schemes, e.g. see [dat-wot](https://github.com/jayrbolton/dat-wot).

## Example

Consider a decentralized Twitter-like app (checkout [fritter](https://github.com/beakerbrowser/fritter)!), where each user has an identity feed to which they publish public "tweets" (or freets)?

If two peers wished to exchange messages privately, they would have to establish a new pair of shared, secret feeds that only they know about, which would normally have to be done with a handshake. (Of course, they could publish encrypted messages to one another on their public feed, but this exposes some information about a user's private messaging behavior).

Using Ristretto, there is a non-interactive, deterministic approach to deriving such a pair of feeds, and as long as both users know of each other's keys, this effectively provides a mechanism for decentralized [push messaging](https://github.com/jayrbolton/dat-wot/issues/7).

Suppose Alice wants to push a message to Bob. Alice can construct a function that takes in hey secret key `a` and Bob's public key `B`, and spits out a secret key `a_to_B`, with associated public key `A_to_B`, and public key `B_to_A`, *such that* using the same function Bob can produce the secret key `b_to_A`, with which he can derive `B_to_A`, and `A_to_B`. Using this pair of keys, Alice and Bob can create an asymmetric pair of relationship feeds that only they know about. (The reason this cannot be done using ed25519 is that standard implementations perform security-related bit-twiddling that compromises the mathematical integrity of keys).

If all users derive such a pair of relationship feeds as they encounter peers on the network (or just peers they wish to receive messages from), then this scheme allows for effective push messages, as long as both users are online at the same time, or have an always-on server replicating their feeds.

Alternatively, if hypercore feeds could be replicated blindly, then users could allow (e.g. trusted) peers to replicate their relationship feeds without exposing any information about the other peer, so that messages can be pushed without both peers being online at the same time, as long as their is at least one replicator online.

To implement push messaging without key derivation would require handshakes between all peers, which on networks with intermittent-connectivity could be slow or on large networks prohibitive.

One downside is that, as a deterministic scheme, such relationship feeds derived by a user would be exposed if their secret key were compromised.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)
