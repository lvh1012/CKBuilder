# Week 3 — 2026-11-15

## Completed

- Docs:
  - [Quick start](https://docs.nervos.org/docs/getting-started/quick-start)
  - [CKB-CLI](https://docs.nervos.org/docs/sdk-and-devtool/ckb-cli)
- Testnet node: [CKB light client](https://github.com/nervosnetwork/ckb-light-client) v0.5.4 on testnet. A full node needs at least 200 GB of free disk and can take days to sync;
- Tutorial #1: [Transfer CKB](https://docs.nervos.org/docs/dapp/transfer-ckb), using the [simple-transfer](https://github.com/nervosnetwork/docs.nervos.org/tree/develop/examples/dApp/simple-transfer) example on **testnet**

## Key learnings

1. **Three networks, three separate states.** Devnet, testnet (Pudge) and mainnet share no balances. The address prefix tells the network: `ckt1…` is testnet, `ckb1…` is mainnet.

2. **Full node vs. light client.**
   - Full node: downloads and verifies every block; needs at least 200 GB of free disk, and syncing from genesis may take days.
   - Light client: verifies the chain from a small sample of block headers, and downloads a full block only when a filter matches the scripts registered with it (about 10 MB of data on testnet). Its RPC (`127.0.0.1:9000`) adds `set_scripts` / `get_scripts`, `fetch_header` and `fetch_transaction`.

3. **JSON-RPC 2.0.** A stateless protocol with JSON messages; the transport is not fixed.
   - **Request:** `jsonrpc` (`"2.0"`), `method`, optional `params` (array or object) and `id`. No `id` means a notification with no response.
   - **Response:** the same `id`, with either `result` or `error` (`code`, `message`, optional `data`). Reserved codes: −32700 Parse error, −32600 Invalid Request, −32601 Method not found, −32602 Invalid params.
   - **Batch:** an array of requests; responses come back as an array in any order, matched by `id`.

4. **CKB JSON-RPC conventions.**
   - **Transports:** HTTP on `127.0.0.1:8114` by default (every call is a `POST`); TCP and WebSocket listeners are optional in `ckb.toml`.
   - **Modules:** methods are grouped into modules (`Chain`, `Pool`, `Net`, `Stats`, `Subscription`, …) that `ckb.toml` enables or disables.
   - **Encoding:** integers are `0x`-prefixed hex strings without leading zeros (`"0x0"`, not `0` or `"0x00"`); hashes are `0x`-prefixed 32-byte hex. Capacity is in shannons: `0x2540be400` = 100 CKB.
   - **`get_transaction`:** `tx_status.status` is `pending`, `proposed`, `committed`, `unknown` or `rejected`. `transaction` is filled only for the first three.
   - **`get_live_cell`:** takes an OutPoint and `with_data`; returns `status: "live"` for an unspent cell.
   - **`send_transaction`:** asynchronous, so a returned hash does not mean the transaction is verified; poll `get_transaction`. Verification errors use CKB codes such as −302 `TransactionFailedToVerify`.

5. **Fees.** Fee = `fee_rate × transaction size / 1000` (rate in shannons per kB) = total input capacity − total output capacity. The size used for the fee is 4 bytes more than the Explorer's Size field.

6. **Reading a transaction on the Explorer.**

   | Field | Meaning |
   |---|---|
   | Block Height / Timestamp | The block that included the transaction and that block's timestamp; a transaction has no timestamp of its own. |
   | Status | Confirmations: blocks built on the chain since inclusion (`committed` in `get_transaction`). |
   | Size | Serialized size of the whole transaction, witnesses included. |
   | Fee / Fee Rate | Fee = total input − total output; rate = fee ÷ size (see item 5). |
   | Cycles | CKB-VM execution cost of the Scripts the transaction runs. |
   | `cell_deps` | Where the node finds Script code; the default lock uses a `dep_group`. |
   | `inputs` | Consumed live cells, identified by OutPoint; `since` `0x0` means no time lock. |
   | `outputs` / `outputs_data` | New cells and their data, matched by index. One output pays the recipient; another returns the change to the sender. |
   | `witnesses` | `WitnessArgs`; its `lock` field holds the 65-byte secp256k1 signature (85 bytes in total), checked against the `args` of the input's lock. |

## Evidence

| # | Step | Command / action | Result | Evidence |
|---|------|------------------|--------|----------|
| 1 | Download and start the light client | `gh release download v0.5.4 -R nervosnetwork/ckb-light-client -p 'ckb-light-client_v0.5.4-x86_64-linux-portable.tar.gz'`, then `./ckb-light-client run --config-file ./config/testnet.toml` | Light client `0.5.4` (git `778590a`, built 2026-01-02) started on testnet. It listens on `/ip4/0.0.0.0/tcp/8118`, connected to 4 peers within 3 seconds (`SessionId(1)` to `SessionId(4)`), then logged `prove_or_download-matched blocks` every 3 seconds. | [light-client-run.png](imgs/light-client-run.png) |
| 2 | Light client is synced | `curl -s http://127.0.0.1:9000/ -X POST -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"get_tip_header","params":[],"id":1}' \| jq` | `get_tip_header` returned block `0x15fd519` (23,057,689), hash `0x22dcc8d8…c1ad86`. The public RPC in #3 reported `0x15fd50a` (23,057,674) about two minutes earlier, so the light client is within 15 blocks of the network tip. | [light-client-tip.png](imgs/light-client-tip.png) |
| 3 | RPC: tip block number | `curl -s -H 'content-type: application/json' -d '{"id":1,"jsonrpc":"2.0","method":"get_tip_block_number","params":[]}' https://testnet.ckb.dev/` | `{"jsonrpc":"2.0","result":"0x15fd50a","id":1}`: tip block 23,057,674. | [rpc-tip.png](imgs/rpc-tip.png) |
| 4 | RPC: get a transaction | `get_transaction` with the Week 2 A2 tx hash (`0x08c579c0…6713a2`) on `https://testnet.ckb.dev/` | Returned the A2 transaction: `cycles` `0x16620e` (1,466,894, the same as the Explorer showed in Week 2); 1 input spending output 0 of `0x7d683800…bda9`; 1 output of `0x218711a00` (90 CKB) with lock `code_hash` `0xf329effd…52cb`; `outputs_data` is `0x`; 2 `cell_deps` (one `dep_group`, one `code`). | [rpc-get-transaction.png](imgs/rpc-get-transaction.png) |
| 5 | RPC: get a live cell | `get_live_cell` with `[{"tx_hash":"<Week 2 A2 tx hash>","index":"0x0"}, true]`, i.e. output #0 of the A2 transaction | `status` is `live`. `block_hash` is `0x3250f0c5…ca00` (the A2 block shown on the Academy page); `output.capacity` is `0x218711a00` (90 CKB) with the same lock as #4; `type` is `null`; `data.content` is `0x`. | [rpc-get-live-cell.png](imgs/rpc-get-live-cell.png) |
| 6 | Create a testnet account | `ckb-cli --url https://testnet.ckb.dev/ account new` | Account created with a password-protected keystore. Testnet address `ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq250dwpexavaxc5qhgeqv2ex23tvp5ljdcsp5cmp`, `lock_arg` `0x547b5c1c9bace9b1405d190315932a2b6069f937`. The output also lists the mainnet (`ckb1…`) form and the deprecated short address format. | [ckb-cli-account.png](imgs/ckb-cli-account.png) |
| 7 | Check the balance | `ckb-cli --url https://testnet.ckb.dev/ wallet get-capacity --address ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq250dwpexavaxc5qhgeqv2ex23tvp5ljdcsp5cmp` | `total: 100000.0 (CKB)`. | [ckb-cli-balance.png](imgs/ckb-cli-balance.png) |
| 8 | Transfer CKB with ckb-cli | `ckb-cli --url https://testnet.ckb.dev/ wallet transfer --from-account ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsq250dwpexavaxc5qhgeqv2ex23tvp5ljdcsp5cmp --to-address ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqf3wjf4602xe36ns88unasrls70y8uvszgj7tfrc --capacity 62 --fee-rate 1000` | It asked for the keystore password, then printed the tx hash `0x0843fa99d074eb6b161ba198dee551a038192ae8bde6abe15f246329a4e1c0fd`. 62 CKB was sent to the `…rls70y8uvszgj7tfrc` address; the fee is in #9. | [ckb-cli-transfer.png](imgs/ckb-cli-transfer.png) |
| 9 | Explorer: ckb-cli transfer | Open the tx hash from #8 on the [Testnet Explorer](https://pudge.explorer.nervos.org/) | Committed in block 23,057,801 with 6 confirmations. Size 460 bytes, 1,645,579 cycles, fee 0.00000464 CKB (1,000 shannons/kB, the rate I passed). Input: 100,000 CKB. Outputs: #0 62 CKB to the recipient, #1 99,937.99999536 CKB change back to my address. Check: 100,000 − 62 − 99,937.99999536 = 0.00000464 CKB. | [explorer-ckb-cli.png](imgs/explorer-ckb-cli.png) |
| 10 | Tutorial #1 on testnet | In `examples/dApp/simple-transfer`: `npm install`, then `npm run start:testnet`; paste the private key of the funded testnet account from #6 (the UI derives the address and balance from it) and send 62 CKB to the app's default recipient, which is the address of OffCKB devnet account #1 | The UI derived address `ckt1qzda…ljdcsp5cmp` and lock `args` `0x547b5c1c…f937`, the same as #6, with `codeHash` `0x9bd7e06f…cce8` (the default secp256k1 lock). Total capacity showed 99937 CKB (the change from #8, decimals dropped, before the post-transfer refresh). Sent 62 CKB; tx hash `0x3c571d6b99ec281a611705824f4eb6c2b601c26887fe218f1e438317201dd463`. | [simple-transfer-testnet.png](imgs/simple-transfer-testnet.png) |
| 11 | Explorer: dApp transfer | Open the tx hash from #10 on the Testnet Explorer | Committed in block 23,057,883 with 5 confirmations. Size 460 bytes, 1,634,954 cycles, fee 0.00000465 CKB (1,002 shannons/kB). Input: 99,937.99999536 CKB, which is output #1 of the #8 transaction. Outputs: #0 62 CKB to the recipient, #1 99,875.99999071 CKB change back to my address. The UI's "Tx fee: 0.001 CKB" label is static text, not the real fee. | [explorer-dapp.png](imgs/explorer-dapp.png) |
