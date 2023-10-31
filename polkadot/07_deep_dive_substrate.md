### Node Types

In the previous chapter, we got an inside look at the substrate node and what it does and in this chapter, we will learn about node types, runtime, transactions, consensus and the default consensus modes.

A substrate node can be of three types - 

#### Full Nodes

Full nodes are a critical part of the blockchain network infrastructure and are the most common node type. Full nodes store blockchain data and, typically, participate in common blockchain operations, such as authoring and validating blocks, receiving and verifying transactions, and serving data in response to user requests.

By default, full nodes are configured to store only the most recent 256 blocks and to discard state older than that—with the exception of the genesis block—to prevent the full node from growing indefinitely and consuming all available disk space.

#### Archive Nodes

Archive nodes are similar to full nodes except that they store all past blocks with complete state available for every block. Archive nodes are most often used by utilities—such as block explorers, wallets, discussion forums, and similar applications—that need access to historical information.

#### Light node clients

Light client nodes enable you to connect to a Substrate network with minimal hardware requirements.

Because light client nodes require minimal system resources, they can be embedded into web-based applications, browser extensions, mobile device applications, or internet of things (IoT) devices. Light client nodes provide a runtime and access to the current state through RPC endpoints.

### Runtime 

In the architecture section, we briefly talked about the runtime, and now it’s time to discuss it in more detail.

The runtime is one of the most important components in a Substrate node and it contains all of the business logic for executing transactions, saving state transitions, and interacting with the outer node. 

Every blockchain records state changes on-chain, in substrate, these changes are recorded in the runtime.

Let’s talk about 2 components of runtime

### FRAME Pallets

FRAME stands for Framework for Runtime Aggregation of Modularized Entities and is one of the most powerful tools available for runtime development as it contains a large number of modules and libraries that simplify runtime development,

In substrate, these modules are called as pallets and these offer customizable business logic for different use cases and features.

 For example, there are pallets that provide a framework of business logic for staking, consensus, governance etc.

#### Building custom pallets

In addition to the pre-built FRAME pallets, you can build your own custom pallets.

With custom pallets, you have the flexibility to define the runtime behavior that best suits your purposes. Because each pallet has its own discrete logic, you can combine pre-built and custom pallets to control the features and functionality your blockchain provides and achieve the results you want.

### Transactions

Transactions provide a mechanism for making changes to state that can be included in a block. By learning how different transaction types are used, we will be better prepared to select the appropriate type for our needs.

There are 3 different types of transactions in substrate - 

#### Signed Transactions

Signed transactions include the signature of an account sending an inbound request to execute some runtime call. The request is signed using the private key for the account that is submitting the request. Usually the account submitting the request also pays a transaction fee. The transaction fees and other elements of transaction processing depend on how the runtime logic is defined.

#### Unsigned Transactions

Unsigned transactions don't require a signature and don't include any information about who submitted the transaction.Unsigned transactions require custom validation, this transaction type consumes more resources than a signed transaction.

#### Inherent Transactions

Inherent transactions are a special type of unsigned transaction. With this type of transaction, block authoring nodes can add information directly to a block. Inherent transactions are not communicated to other nodes or stored in the transaction queue. The data inserted using an inherent transaction is assumed to be valid without requiring specific validation.

### Consensus

Before we learn about consensus and the various consensus algorithms supported in substrate, let’s understand a few important terms.

### Block Authoring

Before you can reach consensus, some nodes in the blockchain network must be able to produce new blocks. 

How the blockchain decides the nodes that are assigned to author blocks depends on which consensus model we use. 

For example, in a centralized network, a single node might author all the blocks. 

In a completely decentralized network without any trusted nodes, an algorithm must select the block author at each block height.

This is called block authoring.

#### Block Finalization and Forks

A blockchain is made of a chain of blocks and each block header contains a reference to its parent block, so you can trace the chain back to the genesis block but forks can occur in a blockchain.

Forks occur when two blocks reference the same parent. Block finalization is a mechanism that resolves forks such that only the canonical chain exists.

A fork choice rule is an algorithm that selects the best chain that should be extended.

For example In the GRANDPA protocol (one of the consensus algorithms), the longest chain rule simply says that the best chain is the longest chain - this is an example of the block finalization mechanism.

### Deterministic Finality

With block authoring, transactions are never entirely finalized. There is always a chance that a longer or heavier chain might revert a transaction. However, the more blocks are built on top of a particular block, the less likely it is to ever be reverted. In this way, block authoring along with a proper fork choice rule provides probabilistic finality.

If your blockchain requires deterministic finality, you can add a finality mechanism to the blockchain logic. For example, you can have members of a fixed authority set, cast finality votes. When enough votes have been cast for a certain block, the block is deemed final. In most blockchains, this threshold is two-thirds. Blocks that have been finalized cannot be reverted without external coordination such as a hard fork.

### Default Consensus Modes

The big benefit with substrate is that you can set your own consensus mechanism for your chain, but by default a substrate chain comes with AURA for block authoring and GRANDPA for block finalization by default. Let’s learn more about these.

#### AURA

AURA stands for Authority based round robin scheduling, it provides a slot-based block authoring mechanism. In Aura a known set of authorities take turns producing blocks.

#### Babe

Stands for Blind assignment of blockchain extension and is also a slot-based block authoring mechanism but does it with a known set of validators and is typically used in proof-of-stake blockchains. The difference when compared to AURA is that BABE is based on the evaluation of a verifiable random function (VRF).

#### Proof Of Work

Is not slot based and does not require a known authority set. In proof of work, anyone can produce a block at any time, so long as they can solve a computationally challenging problem.

#### GRANDPA

GRANDPA provides block finalization. It has a known weighted authority set like BABE. However, GRANDPA does not author blocks. It just listens to gossip about blocks that have been produced by block authoring nodes. GRANDPA validators vote on chains, not blocks,. GRANDPA validators vote on a block that they consider best and their votes are applied transitively to all previous blocks. After two-thirds of the GRANDPA authorities have voted for a particular block, it is considered final.