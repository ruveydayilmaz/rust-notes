### Interchain communication in substrate (cross-consensus messaging)
In Substrate, cross-consensus communication refers to the exchange of information and assets between different Substrate-based blockchain networks that have different consensus mechanisms. Cross-consensus communication is achieved through the use of Inter-Blockchain Communication (IBC) protocols, which provide a standard format for communication between chains and ensure the security and privacy of the transferred data.

Substrate provides a set of modular IBC modules that developers can use to build cross-chain applications. These modules define the rules for cross-consensus communication and allow for the transfer of data and assets between different Substrate-based blockchains, regardless of their underlying consensus mechanism.

For example, a Substrate-based blockchain using proof-of-stake (PoS) consensus can communicate with another Substrate-based blockchain using proof-of-work (PoW) consensus through the use of IBC. The IBC protocols ensure that the data and assets are securely transferred between the two chains, even though they have different consensus mechanisms.

By enabling cross-consensus communication, Substrate promotes greater interoperability between different Substrate-based blockchain networks and helps to facilitate the growth and development of the overall blockchain ecosystem.

Cross-consensus communication relies on a message format—XCM—that is designed to provide a generalized and extensible set of instructions for completing transactions across boundaries created by different consensus systems, transaction formats, and transport protocols.

The XCM format expresses the content of the message. Each message consists of a set of instructions being requested by a sender that can be accepted or rejected by a message recipient. The message format is completely independent of the message protocol used to send and receive messages.

##### There are four important principles you should understand about messages that use the XCM format:
- Messages are asynchronous. After you send a message, there's no expectation that sending system should wait for a response that indicates the message was delivered or executed.
- Messages are absolute in that they are guaranteed to be delivered and interpreted in order and executed efficiently.
- Messages are asymmetric and don't return any results back to the sender. You can only communicate results back to the sender separately using an additional message.
- Messages are agnostic and make no assumptions about the consensus systems between which messages are passed.

### Substrate Core Components
As a blockchain builder's toolkit, Substrate enables you to develop any type of blockchain you can imagine and to define its boundaries based on your application-specific requirements. With this flexibility in mind, one of the decisions you need to make is the type of network you want to build and the role that different nodes might play in that network.

##### Substrate-based blockchains can be used in different types of network architecture. For example, Substrate blockchains are used to build the following network types:
- **Private Networks**: As the name suggests, the networks are restricted to only the nodes that have access to the network. Any custom criteria can be set for the nodes to get access to the private network.
- **Solo Chains**: Solochains are chains that implement their own security protocol and don't connect or communicate with any other chains. Bitcoin and Ethereum are examples of non-Substrate based solo chains.
- **Relay Chains**: Relay chains provide decentralized security and communication for other chains that connect to them. Kusama and Polkadot are examples of relay chains. Relay chains are at the core of Polkadot and all para chains are connected to Relay chains and extend the functionality of this chain.
- **Para Chains**: Para Chains are built to connect to a relay chain and have the ability to communicate with other chains that use the same relay chain. Because parachains depend on the relay chain to finalize the blocks produced, parachains must implement the same consensus protocol as the relay chain they target.
- **Para Threads**: Parathreads are an idea for parachains to temporarily participate (on a block by block basis) in Polkadot security without needing to lease a dedicated parachain slot. This is done through economically sharing the scarce resource of a parachain slot among several competing resources (parathreads).
