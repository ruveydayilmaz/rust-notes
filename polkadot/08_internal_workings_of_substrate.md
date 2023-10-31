### State Transitions and Storage -

#### Key Value Database

Substrate implements its storage database with RocksDB, a persistent key-value store for fast storage environments. It also supports an experimental Parity DB.

The DB is used for all the components of Substrate that require persistent storage, such as:

- Substrate clients
- Substrate light-clients
- Off-chain workers

#### Trie Abstraction

One advantage of using a simple key-value store is that you are able to easily abstract storage structures on top of it.

Substrate uses a Base-16 Modified Merkle Patricia tree ("trie") to provide a trie structure whose contents can be modified and whose root hash is recalculated efficiently.

Tries allow efficient storing and sharing of the historical block state. The trie root is a representation of the data within the trie; that is, two tries with different data will always have different roots. Thus, two blockchain nodes can easily verify that they have the same state by simply comparing their trie roots.

And this concept in eternity is known as “trie abstraction”.

#### Querying Storage

Blockchains that are built with Substrate expose a remote procedure call (RPC) server that can be used to query runtime storage. When you use the Substrate RPC to access a storage item, you only need to provide the key associated with that item. Substrate's runtime storage APIs expose a number of storage item types.

This makes it extremely easy to access or query the storage and is one of the features that makes substrate so popular.

#### Accounts, addresses and keys -

An account represents an identity—usually of a person or an organization—that is capable of making transactions or holding funds. Although accounts are most often used to represent a person, that doesn't have to be the case. 

An account can be used to perform operations on behalf of a user or another entity, or to perform operations autonomously. 

In addition, any single person or entity could have multiple accounts for different purposes and in this section we will learn a bit more about this.

#### Public and Private Keys

In general, every account has an owner who possesses a public and private key pair. The private key is a cryptographically-secure sequence of randomly-generated numbers. The private key generates a random sequence of words called a secret seed phrase or mnemonic. The secret seed phrase is important because it can be used to recover access to an account if the private key is lost.

You may have experienced a seed phrase when you signed up for a wallet such as metamask.

#### Account information in FRAME

An account is usually defined as a public address with a public/private key pair but in runtime, there is a small difference and it’s important for us to know how it works to avoid confusion in the future. In a runtime built with FRAME, an account is defined as a storage map with a 32-byte address identifier and corresponding account information, such as the number of transactions the account has made, the number of modules that depend on the account, and the account balance.

#### Specialized Accounts

With substrate, different chains can implement different rules for how accounts and the keys that control them are used and these are called “specialized” accounts.

In most cases, specialized accounts are implemented in the context of a specific FRAME pallet, either in a prebuilt pallet like Staking or Multisig or in custom pallets that you design.

There are usually two types of specialized accounts - 

- **Multi-signature accounts** - Typically, an account has one and only one owner and that owner holds the private key for signing transactions. The Multisig pallet enables you to configure a specialized account for executing transactions that multiple account owners must approve.
- **Proxy and keyless accounts** - The Proxy pallet provides another way you can configure specialized accounts for a Substrate-based chain using FRAME. With proxy accounts, primary account owners can designate one or more other accounts to act on their behalf. Proxy accounts can be used to add a layer of security by isolating primary account funds from accounts assigned to specific roles that can complete tasks on behalf of the primary account.

### Off Chain operations -

There may be instances where you may want to query offchain data and utilize it and the conventional way of doing this is by using oracles but there is a limitation to the security, scalability and infrastructure efficiency that oracles can provide.

Substrate provides us with a few features to make it easy for us to support offchain operations - 

#### Offchain Workers

Offchain workers enable you to move tasks that might require more time to execute than allowed out of the block processing pipeline. Any task that might take longer than the maximum block execution time allowed is a reasonable candidate for offchain processing such as -

- Website requests
- Random number generation

#### Offchain Storage

Since offchain workers work with data that’s offchain, they need a way to store the information and this is why we have Offchain storage, which is storage that is local to a Substrate node and can be accessed by both offchain workers and on-chain logic:

Offchain workers have both read and write access to offchain storage.On-chain logic has write access through offchain indexing but doesn't have read access.

#### Offchain indexing

On-chain state is expensive because it must be agreed upon and populated to multiple nodes in the network.

Therefore we have offchain index - an optional service that allows the runtime to write directly to the offchain storage independently from offchain workers. The offchain index provides temporary storage for on-chain logic and complements the on-chain state.

### Cross consensus communication

Cross-consensus communication relies on a message format—XCM—that is designed to provide a generalized and extensible set of instructions for completing transactions across boundaries created by different consensus systems, transaction formats, and transport protocols.

The XCM format expresses the content of the message. Each message consists of a set of instructions being requested by a sender that can be accepted or rejected by a message recipient.

#### Message Protocols

In the Polkadot ecosystem, there are three main communication channels—the message protocols—used to transport messages between chains:

- Upward message passing (UMP) enables a parachain to pass messages up to its relay chain.
- Downward message passing (DMP) enables the relay chain to pass messages down to a parachain.
- Cross-consensus message passing (XCMP) enables parachains to exchange messages with other parachains that are connected to the same relay chain.

#### Messages in the XCM format

There are four important properties of the messages that use the XCM format:

- Messages are asynchronous.
- Messages are absolute.
- Messages are asymmetric.
- Messages are agnostic.

### Locations

XCM needs a way to express locations in a general and unambiguous way, for example XCM must be able to identify the location for the following types of activities - 

- where an instruction should be executed.
- where an asset should be withdrawn from.
- where an account to receive assets can be found.

For any of these activities, the location might be in the context of a relay chain, a parachain, a foreign chain, an account on a specific chain, a specific smart contract, or an individual pallet.

### Assets

Most blockchains depend on some type of digital asset to provide economic incentives that are crucial to the security of the network. Some blockchains support a single native asset. Other blockchains allow multiple assets to be managed on-chain, for example, as assets defined in smart contracts or non-native tokens. There are also blockchains that support non-fungible digital assets for one-of-a-kind collectibles. For XCM to support these different types of assets, it must be able to express assets in a general, flexible, and unambiguous way.