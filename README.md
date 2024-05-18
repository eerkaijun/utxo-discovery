# utxo-discovery

## Motivation

UTXO note discovery has historically been a difficult problem to solve for privacy-preserving protocols. For a user to know its balance, the most naive and common way is to use its private key to trial decrypt all the UTXO note commitment that exists. Commitment that can be decrypted by the private key can then be collected and summed up. This is not the ideal solution as once the user go offline or use another client, it will need to repeat the process of trial decryption again. As the usage of the privacy protocols become much larger, the time needed for trial decryption grows linearly.

## Our solution

In recent years there have been many research around improving the UTXO note discovery user experience. Our implementation is a proof-of-concept based on this [post](https://forum.aztec.network/t/note-discovery-proposal-rfp/3247). From a high level, the protocol is as follows:
1. Each user publishes a range [N, N + k] of available indices
2. When a sender would like to make a token transfer to a receiver, they will first established a shared secret x
3. The sender will look for an unused index of the receiver and publish a tag `H(x|index)` to the server along with the transaction
4. Now when the receiver needs to discover the UTXO notes that belongs to it, instead of trial decrypting every single note commitment, the receiver will query the server all the tags within the indices range, which are `H(x|N)` up to `H(x|N+k)`. 
5. The query is done using private information retrieval, with a form of homomorphic encryption and decryption, such that the server does not learn at all which note commitments are queried. Privacy of the user is preserved.

The steps of the private information retrieval can be seen in the `generate_query()` function in [this file](./src/protocol.rs).

## Getting Started

`cargo run` to go through the end to end flow of UTXO note discovery using private information retrieval.