# Week 3 — 2026-11-15

## Completed

- Docs:
  - [Quick start](https://docs.nervos.org/docs/getting-started/quick-start)
  - [CKB-CLI](https://docs.nervos.org/docs/sdk-and-devtool/ckb-cli)
- Testnet node: [CKB light client](https://github.com/nervosnetwork/ckb-light-client) v0.5.4 on testnet. A full node needs at least 200 GB of free disk and can take days to sync;
- Tutorial #1: [Transfer CKB](https://docs.nervos.org/docs/dapp/transfer-ckb), using the [simple-transfer](https://github.com/nervosnetwork/docs.nervos.org/tree/develop/examples/dApp/simple-transfer) example on **testnet**

## Key learnings

1. **Three networks, three separate states.** Devnet, testnet (Pudge) and mainnet share no balances. A devnet account has no CKB on testnet, so a testnet account must be funded with test CKB. The address prefix tells the network: `ckt1…` is testnet, `ckb1…` is mainnet.

2. **Full node vs. light client.**
   - A full node downloads and verifies every block. The docs ask for at least 200 GB of free disk, and syncing from genesis may take up to several days.
   - A light client verifies the chain with limited resources: it downloads only a small sample of block headers, and a full block only when a filter says the block has data for the scripts registered with it. Its data stays small (about 10 MB on testnet).
   - Its RPC listens on `127.0.0.1:9000` and differs from a full node's RPC: it adds `set_scripts` / `get_scripts` to choose which scripts to track, and `fetch_header` / `fetch_transaction`.

3. **JSON-RPC 2.0.** A stateless, lightweight remote-procedure-call protocol with JSON messages; the specification does not fix a transport.
   - **Request:** `jsonrpc` (exactly `"2.0"`), `method`, an optional `params` (an array or an object) and an `id`. A request without an `id` is a notification and gets no response.
   - **Response:** repeats the `id` and carries either `result` or `error`. An error has an integer `code`, a `message` and optional `data`. Reserved codes include −32700 Parse error, −32600 Invalid Request, −32601 Method not found and −32602 Invalid params.
   - **Batch:** an array of requests; the responses come back as an array, in any order, and are matched to the requests by `id`.
   - **Example:** `{"jsonrpc":"2.0","method":"get_tip_block_number","params":[],"id":1}` is answered by `{"jsonrpc":"2.0","result":"0x15fd50a","id":1}`.

4. **CKB JSON-RPC protocols.** CKB adds its own conventions on top of JSON-RPC 2.0:
   - **Transports.** By default the node serves HTTP on `127.0.0.1:8114`; it can also bind TCP and WebSocket listeners (`tcp_listen_address` and `ws_listen_address` in `ckb.toml`, commented out by default, with example ports 18114 and 28114). Over HTTP every call is a `POST` with a JSON body, and a `GET` is rejected. The light client's RPC listens on `127.0.0.1:9000`.
   - **Modules.** Methods are grouped into modules (`Chain`, `Pool`, `Net`, `Stats`, `Subscription` and others) that `ckb.toml` switches on or off.
   - **Encoding.** Integers are `0x`-prefixed hex strings without redundant leading zeros, so a cell index is `"0x0"`; a plain JSON number or `"0x00"` is rejected with error −32602. Hashes are `0x`-prefixed 32-byte hex strings. A capacity of `0x2540be400` is 100 CKB (10^8 shannons per CKB).
   - **`get_transaction`.** It returns `tx_status.status`: `pending` or `proposed` (in the transaction pool), `committed` (in the chain), `unknown` (the node has not seen it) or `rejected` (recently removed from the pool). The `transaction` field holds the transaction only for `pending`, `proposed` and `committed`; otherwise it is `null`.
   - **`get_live_cell`.** It takes an OutPoint and a `with_data` boolean, and reports `status: "live"` for an unspent cell.
   - **`send_transaction`.** It is asynchronous, so a returned hash does not mean the transaction is verified; poll `get_transaction`. Verification failures use CKB's own error codes, for example −302 `TransactionFailedToVerify`.

5. **Fees: the rate matters, not the amount.**
   - A fee is `fee_rate × transaction size / 1000`, with the rate in shannons per kB. It also equals total input capacity minus total output capacity.
   - The size used for the fee is 4 bytes more than the Explorer's Size field (see item 6).

6. **Reading a transaction on the Explorer.** Example: transaction `0x0843fa99d074eb6b161ba198dee551a038192ae8bde6abe15f246329a4e1c0fd` (Evidence #8). The first six rows are on its Explorer page (Evidence #9); the rest are under Parameters / Raw and come from `get_transaction`.

   | Field | Value | What it means |
   |---|---|---|
   | Block Height | 23,057,801 | The block that included the transaction. |
   | Timestamp | 2026/11/14 10:44:59 | The timestamp of that block, in local time. A transaction has no timestamp of its own. |
   | Status | 6 Confirmations | How many blocks have been built on the chain since the transaction was included, as counted by the Explorer. `get_transaction` reports `committed`. |
   | Size | 460 bytes | Serialized size of the whole transaction, witness included. |
   | Transaction Fee / Fee Rate | 0.00000464 CKB / 1,000 shannons/kB | Fee = total input − total output = 100,000 − 62 − 99,937.99999536 CKB (464 shannons). Fee rate = fee ÷ size; it gives 1,000 shannons/kB only with a size 4 bytes larger than the displayed 460 bytes. |
   | Cycles | 1,645,579 | The CKB-VM execution cost of the Scripts the transaction runs; here the lock Script verifying one secp256k1 signature. |
   | `version` | `0x0` | Transaction version. |
   | `cell_deps` | one `dep_group`: `0xf8de3bb4…5d37`, index 0 | Where the node finds the Script code. This dep group holds the `secp256k1_blake160_sighash_all` code. |
   | `header_deps` | empty | No block header is needed. |
   | `inputs` | `0x4b9f5871…114a`, index 0; `since` `0x0` | The live cell being consumed (100,000 CKB), identified by its OutPoint. `since` 0 means no time lock. |
   | `outputs` | #0: 62 CKB, lock `args` `0x3174935d…c809`. #1: 99,937.99999536 CKB, lock `args` `0x547b5c1c…f937` | #0 pays the recipient and #1 is the change back to the sender. Both use the default lock: `code_hash` `0x9bd7e06f…cce8`, `hash_type` `type`, `args` the 20-byte blake160 hash of the owner's public key. Neither has a type Script. 62 CKB is above the 61 CKB minimum for a cell with this lock and no data. |
   | `outputs_data` | `["0x", "0x"]` | Neither output stores data; the entries match the outputs by index. |
   | `witnesses` | 1 witness of 85 bytes | A `WitnessArgs` whose `lock` field holds the 65-byte signature (16-byte header + 4-byte length + 65 bytes). The lock Script checks it against the `args` of the input's lock. |

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
