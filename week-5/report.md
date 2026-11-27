# Week 5 — 2026-11-29

## Completed

- Docs:
  - [Intro to Script](https://docs.nervos.org/docs/script/intro-to-script)
  - [Program Languages for Script](https://docs.nervos.org/docs/script/program-language-for-script)
  - [Rust Script Quick Start](https://docs.nervos.org/docs/script/rust/rust-quick-start)
- Tutorial #5: [Build a Simple Lock](https://docs.nervos.org/docs/dapp/simple-lock), the [simple-lock](src/simple-lock/) example (a JavaScript hash lock on ckb-js-vm) on **devnet**
- Code: [ckb-rust-script](src/ckb-rust-script/), a workspace from [ckb-script-templates](https://github.com/cryptape/ckb-script-templates) with two contracts: `hello-world`, and `simple-lock`, the same hash lock written in Rust, with 8 tests

## Key learnings

1. **A Script is a RISC-V program that returns an exit code.** `0` accepts the transaction; any other value rejects it (nothing committed, no fee). It is deterministic. Off-chain code builds the outputs; the Script only verifies (see Week 2).

2. **Script = `code_hash` + `hash_type` + `args`.** The code lives in a cell listed in `cell_deps`.
   - `data`/`data1`/`data2`: match the binary's Blake2b hash. Immutable, pinned to VM v0/v1/v2.
   - `type`: matches the code cell's Type Script hash. Upgradable, latest VM.
   - `args`: lets many owners share one binary.

3. **Lock runs when its cell is spent. Type runs when its cell is spent or created.** Output Locks never run. A Type Script must check outputs, or a transaction that only creates tokens skips it. Test `test_output_lock_is_never_executed`: an output lock with invalid `args` still passes.

4. **Script groups.** Cells with the same Script (code, hash type, `args`) form one group; the Script runs **once** per group.
   - `GroupInput`/`GroupOutput` = cells of the group. `Input`/`Output` = whole transaction. A Lock's `GroupOutput` is empty.
   - 1 input and 2 same-lock inputs both cost 33,572 cycles; the second input needs no witness.
   - Same on devnet: my unlock spent 2 cells of the same hash lock with one witness and cost 13,084,116 cycles, as did the 1-cell unlock of a previous dry run of this example on the same devnet.
   - Different `args` = different groups, each reads its own witness (wrong witness at index 1 → exit 11).

5. **Syscalls and `ckb-std`.** No OS: a Script reads the transaction with `ecall` (number in `a7`: 93 `exit`, 2052 `load_script`, 2074 `load_witness`, 2177 `debug`).
   - `ckb-std`: `syscalls` (raw) and `high_level` (`load_script`, `load_cell*`, `load_witness_args`, `QueryIter`).
   - Sources: `Input`, `Output`, `CellDep`, `HeaderDep`, `GroupInput`, `GroupOutput`.

6. **Rust Script skeleton.**
   - `ckb_std::entry!(program_entry)` defines `_start` and the panic handler; `program_entry() -> i8` is the exit code.
   - `default_alloc!(16384, 1258306, 64)`: 16 KB fixed + 1.2 MB dynamic heap, so `Vec`/`String` work.
   - VM memory is 4 MB: build with `--release`. Target: `riscv64imac-unknown-none-elf`.
   - `make build` → `build/release/<name>` (deploy this) and `<name>.debug` (~19 MB, debug only).

7. **Exit codes.** `1`–`9` are syscall errors (1 `INDEX_OUT_OF_BOUND`, 2 `ITEM_MISSING`, 3 `LENGTH_NOT_ENOUGH`, 4 encoding), so custom codes start at 10. `simple-lock`: 10 no preimage, 11 hash mismatch, 12 `args` not 32 bytes. No witness → 1 (syscall error forwarded by `?`).

8. **Test and debug.**
   - `ckb-testtool` `Context`: `deploy_cell`, `build_script_with_hash_type`, `create_cell`, `complete_tx`, `verify_tx(&tx, max_cycles)` → cycles, or an error containing `error code N`.
   - A failing transaction is dumped to `tests/failed_txs/*.json`. Replay: `ckb-debugger -f <file> -i 0 -t input -s lock` (`-i 1` for the second group).

9. **Cycles.**
   - Cost: 1 per instruction, 3 branch, 5 multiply, 32 divide, 500 + 0.25/byte syscall, 0.25/byte binary loaded. Limit: `max_block_cycles` = 3.5 billion (mainnet).
   - Hello World 7,182. Rust hash lock 33,572. JS hash lock 13,084,116 (devnet) ≈ **390×** Rust (QuickJS is interpreted in CKB-VM).
   - JavaScript for learning/prototype, Rust for production.

10. **A hash lock proves knowledge of a secret, not ownership.** The preimage is public in `witness.lock` once sent (I decoded `Hello World` on devnet). Anyone can copy it into a competing transaction, so it is unsafe after first use. Use CKB's Blake2b: `ckb-hash` with `default-features = false, features = ["ckb-contract"]`.

11. **JavaScript lock layout.** The lock Script is the ckb-js-vm interpreter. `args` = 2 B + `hash-lock.bc` code hash (32) + its hash type (1) + expected hash (32) = 67 B (`offckb deploy` referenced the code by `data2`, the `04` byte in `args`).
    - Unlock transaction needs 2 `cell_deps`: interpreter and bytecode.
    - Cell capacity = 8 + 32 + 1 + 67 = 108 CKB (61 CKB is the minimum only for the default lock).
    - An address encodes `args`, so each preimage gives a different address.

## Evidence

| # | Step | Command / action | Result | Evidence |
|---|------|------------------|--------|----------|
| 1 | Tool versions | `cargo generate --version`, `ckb-debugger --version`, `rustup target list --installed`, `offckb --version`, `pnpm --version` | cargo-generate 0.26.0; ckb-debugger 1.1.2 (`offckb install ckb-debugger`); targets `x86_64-unknown-linux-gnu`, `riscv64imac-unknown-none-elf`; OffCKB 0.4.13; pnpm 12.3.4. | [tools-versions.png](imgs/tools-versions.png) |
| 2 | Build Hello World | `make build` (after `make generate CRATE=hello-world` and adding `ckb_std::debug!("Hello World!");`) | `build/release/hello-world` 21,440 bytes; `.debug` 19,784,672 bytes. | [rust-hello-build.png](imgs/rust-hello-build.png) |
| 3 | Run Hello World | `ckb-debugger --bin build/release/hello-world`; `make test CARGO_ARGS="-- tests::test_hello_world --nocapture"` | `Script log: Hello World!`, `Run result: 0`, 7,182 cycles (debugger and test). | [rust-hello-run.png](imgs/rust-hello-run.png) |
| 4 | Build Rust hash lock | `make build`, `ls -l build/release`, `cargo fmt --all -- --check` (after `make generate CRATE=simple-lock` and implementing `contracts/simple-lock/src/main.rs`) | `build/release/simple-lock` 51,448 bytes; `.debug` 19,937,808 bytes; `fmt-ok`. | [rust-lock-build.png](imgs/rust-lock-build.png) |
| 5 | Test Rust hash lock | `make test CARGO_ARGS="-- --nocapture --test-threads=1"` | `running 9 tests`, all `ok`. Correct preimage: 33,572 cycles. Wrong preimage: 11. Empty `lock`: 10. No witness: 1. 31-byte `args`: 12. Output lock with invalid `args`: passes. Same-lock inputs: 33,572 cycles for 1 and 2 inputs. Different locks: pass; wrong witness for group B → 11. | [rust-lock-tests.png](imgs/rust-lock-tests.png) |
| 6 | Failing-case test | `make test CARGO_ARGS="-- test_wrong_preimage_is_rejected --nocapture"` | `1 passed; 8 filtered out`; logs `HashMismatch (11)`; the failing tx is written to `tests/failed_txs/0xa59c0f36….json` (replay with `ckb-debugger -f`). | [rust-lock-debugger.png](imgs/rust-lock-debugger.png) |
| 7 | JS tutorial: build | In `src/simple-lock`: `pnpm install`, `pnpm build` | `hash-lock` built; `dist/hash-lock.bc` 27,810 bytes. | [js-build.png](imgs/js-build.png) |
| 8 | JS tutorial: devnet + deploy | `offckb node`; `pnpm run deploy -- --network devnet`, answer `Y` | `tx committed.` Contract OutPoint `0x66e7d89227a1fb692c422e7b4c376b64146ed08cd9a22d563d55e56389439599:0`; ckb-js-vm OutPoint `0x1bb87da347a776a927ab6593e1e10304ca195f8e24279f039008d5e3115b1bf7:15`; deployment files synced to `frontend/deployment/`. | [js-deploy.png](imgs/js-deploy.png) |
| 9 | JS tutorial: frontend + address | In `src/simple-lock/frontend`: `pnpm dev`, open http://localhost:3000, preimage `Hello World` | `DEVNET` / `READY`. Code hash `0xcd262cb3…d23519`; blake2b-256 of `Hello World` `0x106911e4…59ca`. Address `ckt1qzkymvxscq5t5rtnmmy7uhn28sxf3lxle2y4gq4r9pwksr5kfh95vqgqqrxjvt9nnk0g8a372s26263rnqhmdtnehxf78nehrsf044ca6g63jpqsdyg7f7p70y8pavhnn00ly0qaksldttujr8mk8et38zd0yyjeegwmczc8`. Total live capacity already 200.99999506 CKB: change left by that dry run (same preimage, same address). | [js-ui-ready.png](imgs/js-ui-ready.png) |
| 10 | JS tutorial: deposit | `offckb deposit --network devnet <hash-lock address> 300` | Tx hash `0x0aba3867ded11eebb2db02cdddbc0fdfb92eab12a3ff8953cd3da34b952c45d3`; UI shows Total live capacity 500.99999506 CKB (200.99999506 + 300). | [js-deposit.png](imgs/js-deposit.png), [js-ui-funded.png](imgs/js-ui-funded.png) |
| 11 | JS tutorial: correct preimage | Preimage `Hello World`, send 99 CKB, Transfer CKB | `committed`; tx hash `0x5440aa72b7fbf32fbd9dd69028d4b064b6f91400962c57112d51cac1f8fbb488`. | [js-ui-unlock.png](imgs/js-ui-unlock.png) |
| 12 | JS tutorial: inspect unlock tx | `curl` `get_transaction` (hash from #11) on `http://127.0.0.1:8114`, decode the witness with `python3` | `committed` in block 689 (`0x2b1`); 13,084,116 cycles. 2 inputs: `0x31c6bcee…8c04:1` (change of that dry run) and `0x0aba3867…45d3:0` (the deposit). Outputs: 99 CKB (default lock `0x9bd7e06f…cce8`) and 401.99998968 CKB back to the hash lock. Fee 538 shannons. 2 `cell_deps`: `0x66e7d892…9599:0` (`hash-lock.bc`) and `0x1bb87da3…1bf7:0xf` (ckb-js-vm). One witness ending `48656c6c6f20576f726c64` = `Hello World`. | [js-rpc-unlock-tx.png](imgs/js-rpc-unlock-tx.png) |
