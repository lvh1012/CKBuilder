# Week 4 — 2026-11-22

## Completed

- [Docs (CCC)](https://docs.ckbccc.com/en/docs/getting-started/introduction)
- [CCC Playground](https://live.ckbccc.com/) 
- Tutorial #2: [Store Data on Cell](https://docs.nervos.org/docs/dapp/store-data-on-cell)
- Tutorial #3: [Create Fungible Token](https://docs.nervos.org/docs/dapp/create-token)
- Code (stretch): [ccc-next-app](src/ccc-next-app/), a Next.js app that connects a wallet with `@ckb-ccc/connector-react`, shows the balance and transfers CKB on testnet

## Key learnings

1. **CCC is a stack of packages.** `@ckb-ccc/core` holds Client, Signer, Transaction and Address. The wallet packages sit above it: `@ckb-ccc/connector-react` → `@ckb-ccc/connector` → `@ckb-ccc/ccc` → one package per wallet (`joy-id`, `eip6963`, `nip07`, `okx`, …) → `core`. `@ckb-ccc/shell` is the package for Node.js backends.

2. **Client and Signer.**
   - `Client`: JSON-RPC and indexer queries (`getCellLive`, `getBalance`, `findCellsByType`, `waitTransaction`). `ClientPublicTestnet` / `ClientPublicMainnet` are built in; devnet is `ClientPublicTestnet` with the OffCKB proxy URL (`http://localhost:28114`) and the devnet system scripts.
   - `Signer`: `getRecommendedAddress`, `getBalance`, `signTransaction`, `sendTransaction`. `SignerCkbPrivateKey` signs with a raw private key; in React, `ccc.useSigner()` returns a wallet signer. Transaction code is the same for both.

3. **Composing a transaction.** Declare `outputs` → `completeInputsByCapacity(signer)` → `completeFeeBy(signer)` → `sendTransaction`. `completeFeeBy(signer, 1000)` fixes the rate at 1000 shannons per kB (the rate of the Week 3 `ckb-cli` transfer); without the second argument it uses the network rate.

4. **Capacity is filled automatically: 61 + data bytes.** With `capacity` unset, CCC sets the minimum: 8 (capacity) + 53 (lock: `code_hash` 32 + `hash_type` 1 + `args` 20) + data length.

5. **Data lives in `outputsData`, matched to `outputs` by index.** `getCellLive(outPoint, true)` returns the data; the `true` is the `with_data` flag of `get_live_cell` in Week 3, and without it `outputData` is `0x`. A cell is live only after its transaction is committed.

6. **xUDT cell.** Lock: the holder. Type: the xUDT Script from `KnownScript.XUdt`, `args` = the issuer's lock hash (32 bytes) + flags `00000000` (4 bytes). Data: the amount as a uint128 little-endian number (`numLeToBytes(amount, 16)`, 16 bytes). Each token cell is 146 CKB = 8 + 53 (lock) + 69 (type: 32 + 1 + 36) + 16 (data). The transaction needs the xUDT cell dep (`addCellDepsOfKnownScripts`).

7. **Token transfer.** `completeInputsByUdt` picks the sender's token cells; `getInputsUdtBalance − getOutputsUdtBalance` is the change, returned to the sender as a new token cell. A balance is the sum of the amounts in the live cells with the type script (`findCellsByType`); there is no balance table.

8. **Wallet connection in React.** `ccc.Provider` at the root (a client component in the Next.js App Router); `ccc.useCcc()` gives `open`, `disconnect`, `wallet`; `ccc.useSigner()` gives the signer or `undefined`. Wallet signers use their own lock Scripts, not the secp256k1 lock of a raw private key.

## Evidence

| # | Step | Command / action | Result | Evidence |
|---|------|------------------|--------|----------|
| 1 | Reuse the Week 3 testnet account | `ckb-cli --url https://testnet.ckb.dev/ wallet get-capacity --address ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq250dwpexavaxc5qhgeqv2ex23tvp5ljdcsp5cmp` | Same address as Week 3 (`lock_arg` `0x547b5c1c…f937`); `total: 99875.99999071 (CKB)`, equal to the balance after the Week 3 dApp transfer. | [testnet-account-balance.png](imgs/testnet-account-balance.png) |
| 2 | Playground: Build a CKB transfer | Load source `packages/examples/src/transfer.ts` on **TESTNET** with a MetaMask wallet (`0xe8d…9a5d`), Run | Run complete, 4 outputs. Final snapshot: 2 inputs of 100 CKB = 200 CKB; outputs 100 CKB and 99.99981096 CKB = 199.99981096 CKB; fee 0.00018904 CKB (33223 shannons/KB) after `completeFeeBy`; 2 cell deps, 1 witness. | [pg-1-transfer.png](imgs/pg-1-transfer.png) |
| 3 | Playground: Build an xUDT transfer | Load source `transferUdt.ts`, Run | Run failed. Snapshot before the error: 0 inputs, 1 output of 144 CKB (typed cell, 16 bytes), fee "Not available". Error from `completeInputsByUdt`: `Insufficient coin, need 100000000 extra coin`. | [pg-2-xudt.png](imgs/pg-2-xudt.png) |
| 4 | Playground: Query and transfer a Spore | Load source `transferSpore.ts`, Run | `Found 0 Spore(s)`; `No Spores found. Create one first using createSpore example.` | [pg-3-spore.png](imgs/pg-3-spore.png) |
| 5 | Playground: Query balances | Load source `queryBalance.ts`, Run | Balance via signer: 100990 CKB; via client (`getBalanceSingle` of the recommended address): 0 CKB; total balance (`getBalance` of all signer addresses): 100990 CKB. | [pg-4-balances.png](imgs/pg-4-balances.png) |
| 6 | Playground: Explore live cells | Load source `queryCells.ts`, Run | `Found 0 cells (showing max 10)` by the recommended lock; `Found 0 UDT cells`; `Signer owns at least 11 cells`. | [pg-5-cells.png](imgs/pg-5-cells.png) |
| 7 | Playground: Read transaction history | Load source `queryTransactions.ts`, Run | `Found 0 transactions (showing max 10)` for the recommended address. | [pg-6-history.png](imgs/pg-6-history.png) |
| 8 | Playground: Resolve a CKB DID | Load source `resolveDid.ts`, Run | `No live cell for did:ckb:qq2m72a2vas4e5ovcpxoedscguuu4nba`; `Address controls 0 DID(s):`. | [pg-7-did.png](imgs/pg-7-did.png) |
| 9 | Playground: Trace DID history | Load source `didHistory.ts`, Run | `No history for did:ckb:qq2m72a2vas4e5ovcpxoedscguuu4nba; DID does not exist on this network.` | [pg-8-did-history.png](imgs/pg-8-did-history.png) |
| 10 | Tutorial #2 on testnet: write and read a message | In `src/store-data-on-cell`: `npm install`, `npm run start:testnet`; paste the funded testnet key; write `Hello, world!`, then Read | Network testnet; address `ckt1qzda…jdcsp5cmp`, lock `args` `0x547b5c1c…f937`, total capacity 99875 CKB. Tx hash `0xe0a222e48b93cac4c9d2e5c2a9d0af67400037684353edd6ca37b2d5c5063bdf`. Read returned `Message: Hello, world!`. | [store-write.png](imgs/store-write.png) |
| 11 | Tutorial #3: issue a token | In `src/xudt`: `npm run start:testnet`; Issue 1000 | Balance before: 99656 CKB; lock script hash `0xac89248b…5d7627`. Tx hash `0xb489ba7a6798657e5500d049b8723fda496176829289e87f7a4e2e2a72ea10cf`. xUDT args `0xac89248b100d7f0db151a4b9d09c63ac1a603fa69cd26c99fe210dfb7a5d762700000000` (lock script hash + `00000000`, 36 bytes). Token cell: 146 CKB; type `code_hash` `0x25c29dc3…0975bb`, `hash_type` `type`. | [xudt-issue.png](imgs/xudt-issue.png) |
| 12 | Tutorial #3: query the token | Wait for commit, then Query Issued Token with the xUDT args from #10 | Cell #0: token amount 1000; token holder's lock script args `0x547b5c1c9bace9b1405d190315932a2b6069f937`. | [xudt-query.png](imgs/xudt-query.png) |
| 13 | Tutorial #3: transfer | Transfer Custom Token: 300 of the token from #10 to another testnet address | The UI returned the transaction JSON (`version` `0x0`, `cellDeps` with a `code` dep). Console: `balanceDiff: 700n`; `The transaction hash is 0x283601f0f4c5f88124ed545e724c345ab91181156f8ac24ae56b312cd7ece9e0`. | [xudt-transfer.png](imgs/xudt-transfer.png) |
| 14 | Next.js: connect a wallet | In `src/ccc-next-app`: `npm install`, `npm run dev`; open http://localhost:3000, Connect wallet | Wallet MetaMask; address `ckt1qrejnmlar3r452…jaqqt25t0a`; balance 100990 CKB. | [next-connect.png](imgs/next-connect.png) |
| 15 | Next.js: transfer CKB | Send 62 CKB to another testnet address, approve in the wallet | Status "Committed."; tx hash `0xec43af85bd7f5adc92463261b9a080be740ca8717c9be5dcccaf6af79e2e5458`; balance after: 100927.99981163 CKB. Explorer: block 23,148,051, 7 confirmations, size 563 bytes, 1,433,310 cycles, fee 0.00018837 CKB (33,222 shannons/kB). Inputs: 2 × 100 CKB. Outputs: #0 62 CKB to `ckt1qzda0cr0…eejmk5xgw6jcfw7zmg`, #1 137.99981163 CKB change. Check: 200 − 62 − 137.99981163 = 0.00018837 CKB. `npm run lint` and `npm run build` pass. | [next-transfer.png](imgs/next-transfer.png), [explorer-next-transfer.png](imgs/explorer-next-transfer.png) |
