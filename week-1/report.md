# Week 1 — 2026-10-31

## Completed

- Academy:
  - [Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf)
  - [Nervos Nation videos](https://youtu.be/Jd7_tg4W1zo) on UTXO/PoW and the Cell Model
  - [Introduction to Nervos CKB](https://docs.nervos.org/docs/ckb-fundamentals/nervos-blockchain)
  - [A1: CKB basic theoretical knowledge](https://academy.ckb.dev/courses/basic-theory)
- Code: [simple-transfer](src/simple-transfer/), a React dApp built with CCC that runs on devnet

## Key learnings

1. **Trust.** A blockchain is a distributed, append-only ledger. Nodes agree on its state through consensus, so no trusted third party is needed.

2. **Hash functions.** Map any input to a fixed-length output (256 bits for SHA-256). They are deterministic, preimage resistant, collision resistant, and show the avalanche effect (a small input change flips the output). CKB uses Blake2b-256.

3. **Keys and addresses.** A private key is a secret random number. The public key is derived from it through a one-way elliptic-curve operation. On CKB, an address encodes a lock script (`code_hash` + `args`). For the default lock, `args` is the Blake2b-160 hash of the public key. The default lock uses secp256k1, as Bitcoin and Ethereum do.

4. **Signatures.** `sign(message, private_key)` and `verify(message, signature, public_key)`. A valid signature proves who signed, that the content is unchanged, and that the signer cannot deny signing.

5. **Blocks and Merkle trees.** The header holds `prev_hash`, `merkle_root`, `timestamp`, `nonce`, and `difficulty`. The body holds the transactions. The Merkle root lets anyone verify that a transaction is in a block without downloading the whole block.

6. **Chaining.** Each block stores the previous block's hash, so editing one transaction breaks every later link. Hashing is cheap to redo, so the chain also needs Proof of Work.

7. **Proof of Work.** Miners search for a `nonce` that puts the header hash below a target. Finding it takes many attempts; verifying it takes one hash. Bitcoin uses double SHA-256. CKB uses Eaglesong and adjusts difficulty automatically.

8. **Double-spend.** Nodes follow the chain with the most cumulative work. Reversing a transaction buried under many blocks requires a longer chain than the honest network, which means a large share of total hash power (a 51% attack).

9. **Account vs. UTXO.**
   - Account model (bank): a global table of balances per address.
   - UTXO model (cash): a transaction consumes unspent outputs and creates new ones. Inputs must be at least outputs; the difference is the miner fee. A spent UTXO cannot be spent again, so double-spending is rejected by design.

10. **Nervos CKB.** CKB is the Layer 1 base ledger; Layer 2 systems build on top of it for scaling. CKByte is the native token, and 1 CKByte covers 1 byte of cell capacity. Overhead counts too: the 8-byte capacity field, the lock script, and the type script. The smallest cell with the default secp256k1 lock needs 61 CKByte, even with no data.

## Evidence

| # | Step | Command / action | Result | Evidence |
|---|------|------------------|--------|----------|
| 1 | Install Node.js LTS | `nvm install --lts`, then `node -v`, `npm -v` | Node v26.13.0, npm v11.20.0 | [install-node.png](imgs/install-node.png) |
| 2 | Install Rust | rustup; check with `rustc --version`, `cargo --version` | rustc 1.98.1, cargo 1.98.1 | [install-rust.png](imgs/install-rust.png) |
| 3 | Install OffCKB | `npm i -g @offckb/cli`, then `offckb --help` | 134 packages added; help lists all commands | [install-offckb.png](imgs/install-offckb.png) |
| 4 | Run local devnet | `offckb node` | CKB 0.208.0 installed; devnet at http://127.0.0.1:8114 | [run-offckb-node.png](imgs/run-offckb-node.png) |
| 5 | List built-in accounts | `offckb accounts` | Accounts #0–#2 listed, each funded with 42,000,000 CKB in the genesis block | [run-offckb-accounts.png](imgs/run-offckb-accounts.png) |
| 6 | *(Stretch)* Connect CKB AI MCP | `claude mcp add --transport http ckb-ai https://mcp.ckbdev.com/ckbai` | Server added to local Claude config | [add-ckb-ai.png](imgs/add-ckb-ai.png) |
| 7 | Run simple-transfer | In `week-1/src/simple-transfer`: `npm install`, then `npm start` (with `offckb node` running) | Devnet system scripts synced; app served at http://localhost:1234 | [run-simple-transfer.png](imgs/run-simple-transfer.png) |
| 8 | Transfer CKB in the UI | Enter a recipient address and amount of 62 CKB, click Transfer | Tx sent, UI shows fee 0.001 CKB, tx hash `0x80f6485e3ce19f075420c460604eb17ecb8b2e439c96b5bb761d22791d419e2f` | [simple-transfer-ui.png](imgs/simple-transfer-ui.png) |
| 9 | Check balances after transfer | `offckb balance` for recipient (#1) and sender (#0) | Recipient 42,000,062 CKB (+62); sender 41,999,937.99999535 CKB (−62.00000465 = 62 sent + 0.00000465 CKB fee) | [simple-transfer-result.png](imgs/simple-transfer-result.png) |
