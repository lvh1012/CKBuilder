# Week 1 — 2026-10-31

## Completed

- Academy:
  - [Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf)
  - [Nervos Nation videos](https://youtu.be/Jd7_tg4W1zo) on UTXO/PoW and the Cell Model
  - [Introduction to Nervos CKB](https://docs.nervos.org/docs/ckb-fundamentals/nervos-blockchain)
  - [A1: CKB basic theoretical knowledge](https://academy.ckb.dev/courses/basic-theory)
- Tutorials / Code: None this week. No commits or on-chain transactions yet.

## Key learnings

1. **The core problem is trust.** A blockchain is a distributed, append-only ledger. Independent nodes agree on its state through a consensus rule, so the parties do not need a trusted third party.

2. **Hash functions.** A hash function maps any input to a fixed-length output (256 bits for SHA-256). Its key properties:
   - Deterministic: the same input always gives the same output.
   - Preimage resistance: the input cannot be recovered from the hash.
   - Collision resistance: two inputs with the same hash are very hard to find.
   - Avalanche effect: a small change in the input produces a large change in the output, so any edit is immediately visible.

   CKB uses Blake2b-256 instead of SHA-256.

3. **Asymmetric keys.** A private key is a large random number and must stay secret. The public key is derived from it through a one-way elliptic-curve operation. On CKB, an address encodes a lock script (`code_hash` + `args`). For the default lock, `args` is the Blake2b-160 hash of the public key. CKB's default lock uses the secp256k1 curve, the same as Bitcoin and Ethereum.

4. **Digital signatures.** `sign(message, private_key)` produces a signature, and `verify(message, signature, public_key)` checks it. A valid signature guarantees authenticity (the right signer), integrity (the content was not changed), and non-repudiation (the signer cannot deny signing).

5. **Block structure and Merkle trees.** A block has a header (`prev_hash`, `merkle_root`, `timestamp`, `nonce`, `difficulty`) and a body (the list of transactions). The Merkle root is the top hash of a tree built from pairwise hashes of the transactions. It lets anyone prove that a transaction is in a block without downloading the whole block.

6. **Chaining makes history hard to rewrite.** Each block stores the hash of the previous block. Changing one transaction changes that block's hash, which breaks the link in the next block, and so on up to the tip. Hashing alone is cheap to redo, so the chain also needs real computational cost, which is what Proof of Work adds.

7. **Proof of Work (PoW).** A miner must find a `nonce` such that the block header's hash is below a target. Finding it takes many attempts, but verifying it takes one hash. Bitcoin uses double SHA-256 (hashing twice). CKB uses the Eaglesong algorithm and adjusts difficulty automatically.

8. **Double-spend prevention.** Two conflicting transactions that spend the same coin end up on different branches. Nodes accept the chain with the most cumulative work. To reverse a transaction buried under many blocks, an attacker would need to build a longer chain than the honest network. This requires a large share of the total hash power (a 51% attack).

9. **Account vs. UTXO model.**
   - Account model (like a bank): a global state table stores a balance for each address.
   - UTXO model (like cash): the system tracks unspent transaction outputs. A transaction consumes its inputs and creates new outputs. The sum of inputs must be at least the sum of outputs, and the difference is the fee paid to the miner.
   - A spent UTXO cannot be spent again, so double-spending is rejected naturally.

10. **Nervos CKB.** CKB is the Layer 1 base ledger. Layer 2 systems are built on top of it for scaling. CKByte is the native token, and each CKByte covers one byte of on-chain state. To store 100 bytes of state, 100 CKByte must be locked in that cell. The cell's own fields also count toward its capacity (the 8-byte capacity field, the lock script, and the type script). The smallest cell with the default secp256k1 lock needs 61 CKByte, even with no data. The Cell Model is how CKB represents this state.

## Evidence

_Screenshots to be added._
