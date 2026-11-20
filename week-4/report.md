# Week 4 — 2026-11-22

## Completed

- Docs (CCC): [Introduction](https://docs.ckbccc.com/en/docs/getting-started/introduction), [Quick start](https://docs.ckbccc.com/en/docs/getting-started/quick-start), [Installation](https://docs.ckbccc.com/en/docs/getting-started/installation); concepts [Client](https://docs.ckbccc.com/en/docs/concepts/client), [Signer](https://docs.ckbccc.com/en/docs/concepts/signer), [Transaction](https://docs.ckbccc.com/en/docs/concepts/transaction), [Address](https://docs.ckbccc.com/en/docs/concepts/address); guides [Compose Transactions](https://docs.ckbccc.com/en/docs/guides/compose-transactions), [UDT Tokens](https://docs.ckbccc.com/en/docs/guides/udt-tokens), [Connect Wallets](https://docs.ckbccc.com/en/docs/guides/connect-wallets), [Node.js Backend](https://docs.ckbccc.com/en/docs/guides/node-js-backend). Three CCC links in the syllabus (`/docs/CCC`, `/docs/ccc-app`, `/docs/playground`) return 404, so I used the `/en/docs/` pages listed in the site's [llms.txt](https://docs.ckbccc.com/llms.txt).
- [CCC Playground](https://live.ckbccc.com/): all 8 built-in examples on testnet, plus one real transaction sent with `signer.sendTransaction`.
- Tutorial #2: [Store Data on Cell](https://docs.nervos.org/docs/dapp/store-data-on-cell), using the [store-data-on-cell](src/store-data-on-cell/) example on **testnet**
- Tutorial #3: [Create Fungible Token](https://docs.nervos.org/docs/dapp/create-token), using the [xudt](src/xudt/) example on **testnet**
- Code (stretch): [ccc-next-app](src/ccc-next-app/), a Next.js app that connects a wallet with `@ckb-ccc/connector-react`, shows the balance and transfers CKB on testnet

## Key learnings

1. **CCC is a stack of packages.** `@ckb-ccc/core` (Client, Signer, Transaction, Address) is all the two tutorials import. The wallet packages sit above it: `@ckb-ccc/connector-react` → `@ckb-ccc/connector` → `@ckb-ccc/ccc` → one package per wallet (`joy-id`, `eip6963`, `nip07`, `okx`, …) → `core`. The Node.js Backend guide uses `@ckb-ccc/shell`.

2. **Client reads the chain, Signer owns the authority to spend.**
   - `Client`: JSON-RPC and indexer queries (`getCellLive`, `getBalance`, `findCellsByType`, `waitTransaction`). `ClientPublicTestnet` / `ClientPublicMainnet` are built in; devnet is `ClientPublicTestnet` with the OffCKB proxy URL (`http://localhost:28114`) and the devnet system scripts.
   - `Signer`: `getRecommendedAddress`, `getBalance`, `signTransaction`, `sendTransaction`. The tutorials use `SignerCkbPrivateKey`; the Next.js app gets a wallet signer from `ccc.useSigner()`.
   - The transaction code is identical for both kinds of signer, because it only calls the Signer interface.

3. **The order of composing a transaction is fixed.** Declare `outputs` → `completeInputsByCapacity(signer)` → `completeFeeBy(signer)` → `sendTransaction`. The fee depends on the final size, which depends on the inputs and the change output, so calling `completeFeeBy` first gives a wrong fee.

4. **Capacity is filled automatically and equals 61 + data bytes.** The tutorials leave `capacity` unset and CCC sets the minimum the cell needs: 8 (capacity) + 53 (lock: `code_hash` 32 + `hash_type` 1 + `args` 20) + the data length. On devnet, messages of 2, 13, 38 and 100 bytes gave cells of 63, 74, 99 and 161 CKB. `xin chào CKB` is 13 bytes, not 12, because `à` takes 2 bytes in UTF-8.

5. **Fee.** `completeFeeBy(signer, 1000)` fixes the rate at 1000 shannons per kB, which is the same rate as the Week 3 `ckb-cli` transfer; leaving the rate out uses the network rate. The fee is still `rate × size / 1000` = inputs − outputs.

6. **Data lives in `outputsData`, matched to `outputs` by index.** Tutorial #2 puts the UTF-8 message into `outputsData[0]`. Reading it back needs `getCellLive(outPoint, true)`: the `true` is the same `with_data` flag as `get_live_cell` in Week 3, and without it the data is not returned. The cell is not live until the transaction is committed, so the app polls.

7. **An xUDT cell has three parts.** The lock is the holder; the type is the xUDT Script from `KnownScript.XUdt`, with `args` = the issuer's lock hash (32 bytes) + flags `00000000` (4 bytes); the data is the amount as a uint128 little-endian number (`numLeToBytes(amount, 16)`, 16 bytes). On devnet each token cell is 146 CKB = 8 + 53 (lock) + 69 (type: 32 + 1 + 36) + 16 (data). The transaction also needs the xUDT cell dep (`addCellDepsOfKnownScripts`). Because the `args` carry the issuer's lock hash, the issuer can mint more with that lock; Week 6 covers that rule (owner mode).

8. **A token transfer consumes cells and creates new ones.** `completeInputsByUdt` picks the sender's token cells; `getInputsUdtBalance − getOutputsUdtBalance` is the change, which goes back to the sender as a new token cell. On devnet, issuing 1000 and sending 300 produced a 300 cell for the receiver and a 700 cell for the issuer. There is no balance table: a balance is the sum of the amounts in the live cells that carry the type script (`findCellsByType`). Every token cell also locks CKB capacity, so a token transfer costs CKB too.

9. **Playground.** It builds and previews; it never signs or sends on its own. A top-bar toggle selects Testnet or Mainnet; the signer is a connected wallet or a READ-ONLY example wallet, and a run with no wallet stops with "no wallet was connected". **Run** goes through all `render(tx)` checkpoints, **Step** pauses at each one. The transaction snapshot shows Inputs, Outputs and the fee, which is "Not available" until inputs and fee are completed. To broadcast, add `signer.sendTransaction(tx)` and use a funded wallet.

10. **Wallet connection in React.** `ccc.Provider` goes at the root (a client component in the Next.js App Router); `ccc.useCcc()` gives `open`, `disconnect`, `wallet`; `ccc.useSigner()` gives the signer or `undefined`. The page never sees a private key. The modal lists Khie, JoyID Passkey, MetaMask, OKX Wallet, UniSat and UTXO Global Wallet. These wallets use other lock Scripts, so their address differs from the secp256k1 address of the tutorial key.

11. **Two traps in the tutorial apps.** `npm start` runs on devnet and refreshes `system-scripts.json` from the local OffCKB (`npm run sync:devnet`); `npm run start:testnet` switches `NETWORK` to testnet. The xUDT example logs a link to the testnet Explorer even when it runs on devnet; it is a fixed string, like the "Tx fee: 0.001 CKB" label in Week 3.

## Evidence

| # | Step | Command / action | Result | Evidence |
|---|------|------------------|--------|----------|
| 1 | Create a testnet key and fund it | One `node` command with `@ckb-ccc/core` writes a random key to the git-ignored `.env` and prints the address; claim CKB at the [Faucet](https://faucet.nervos.org/) | Address `TODO(user)`; balance `TODO(user)` CKB. | [testnet-key-faucet.png](imgs/testnet-key-faucet.png) |
| 2 | Playground 1: Build a CKB transfer | Examples → Build a CKB transfer, Step | Inputs `TODO(user)` CKB, outputs `TODO(user)` CKB, fee `TODO(user)` after `completeFeeBy`. | [pg-1-transfer.png](imgs/pg-1-transfer.png) |
| 3 | Playground 2: Build an xUDT transfer | Examples → Build an xUDT transfer, Step | `TODO(user)`: preview, or the error shown when the wallet has no sample token. | [pg-2-xudt.png](imgs/pg-2-xudt.png) |
| 4 | Playground 3: Query and transfer a Spore | Examples → Query and transfer a Spore, Run | `TODO(user)`: empty result if the wallet owns no Spore. | [pg-3-spore.png](imgs/pg-3-spore.png) |
| 5 | Playground 4: Query balances | Examples → Query balances, Run | Signer and client balance `TODO(user)` CKB. | [pg-4-balances.png](imgs/pg-4-balances.png) |
| 6 | Playground 5: Explore live cells | Examples → Explore live cells, Run | `TODO(user)` live cells found by lock; xUDT balance `TODO(user)`. | [pg-5-cells.png](imgs/pg-5-cells.png) |
| 7 | Playground 6: Read transaction history | Examples → Read transaction history, Run | `TODO(user)` transactions listed; latest block `TODO(user)`. | [pg-6-history.png](imgs/pg-6-history.png) |
| 8 | Playground 7: Resolve a CKB DID | Examples → Resolve a CKB DID, Run | `TODO(user)`. | [pg-7-did.png](imgs/pg-7-did.png) |
| 9 | Playground 8: Trace DID history | Examples → Trace DID history, Run | `TODO(user)`: empty history if the DID is absent. | [pg-8-did-history.png](imgs/pg-8-did-history.png) |
| 10 | Playground: send a real transaction | Example 1 plus `const txHash = await signer.sendTransaction(tx);` with a funded wallet | Tx hash `0xTODO(user)`; committed in block `TODO(user)`; fee `TODO(user)` CKB. | [pg-send.png](imgs/pg-send.png), [explorer-pg-send.png](imgs/explorer-pg-send.png) |
| 11 | Tutorial #2 on testnet: write a message | In `src/store-data-on-cell`: `npm install`, `npm run start:testnet`; paste the funded testnet key; save `hi` | Tx hash `0xTODO(user)`. | [store-write.png](imgs/store-write.png) |
| 12 | Tutorial #2: read it back | Click Read after the transaction is committed | The UI shows `hi` read from output #0 of the transaction. `TODO(user)`: confirm. | [store-read.png](imgs/store-read.png) |
| 13 | Tutorial #2: capacity experiment | Save messages of 2, 13, 38 and 100 bytes; open each transaction on the Explorer | Output #0 capacity: devnet 63, 74, 99 and 161 CKB (= 61 + bytes); testnet `TODO(user)`. | [explorer-store-short.png](imgs/explorer-store-short.png), [explorer-store-long.png](imgs/explorer-store-long.png) |
| 14 | Tutorial #3: issue a token | In `src/xudt`: `npm run start:testnet`; Issue 1000 | Tx hash `0xTODO(user)`; xUDT args `0xTODO(user)` (36 bytes). | [xudt-issue.png](imgs/xudt-issue.png) |
| 15 | Explorer: issue transaction | Open the tx hash from #14 | Output #0: type is the xUDT Script, data is amount 1000 in little-endian, capacity devnet 146 CKB / testnet `TODO(user)`. | [explorer-xudt-issue.png](imgs/explorer-xudt-issue.png) |
| 16 | Tutorial #3: query the token | Wait for commit, then Query Issued Token with the xUDT args from #14 | `TODO(user)`: cell(s) found, amount 1000. | [xudt-query.png](imgs/xudt-query.png) |
| 17 | Tutorial #3: transfer | Transfer 300 to another testnet address | Tx hash `0xTODO(user)`. Output #0: 300 to the receiver; output #1: 700 change to the issuer (devnet check: same). | [xudt-transfer.png](imgs/xudt-transfer.png), [explorer-xudt-transfer.png](imgs/explorer-xudt-transfer.png) |
| 18 | Next.js: connect a wallet | In `src/ccc-next-app`: `npm install`, `npm run dev`; open http://localhost:3000, Connect wallet | The modal lists Khie, JoyID Passkey, MetaMask, OKX Wallet, UniSat, UTXO Global Wallet. Wallet used: `TODO(user)`. | [next-connect.png](imgs/next-connect.png) |
| 19 | Next.js: address and balance | After connecting | Address `TODO(user)`; balance `TODO(user)` CKB. | [next-balance.png](imgs/next-balance.png) |
| 20 | Next.js: transfer CKB | Send 62 CKB to another testnet address, approve in the wallet | Status "Committed."; tx hash `0xTODO(user)`; fee `TODO(user)` CKB. `npm run lint` and `npm run build` pass. | [next-transfer.png](imgs/next-transfer.png), [explorer-next-transfer.png](imgs/explorer-next-transfer.png) |
