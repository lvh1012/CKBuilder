# Week 5 — 2026-11-29

## Completed

- Docs:
  - [Intro to Script](https://docs.nervos.org/docs/script/intro-to-script)
  - [Program Languages for Script](https://docs.nervos.org/docs/script/program-language-for-script)
  - [Rust Script Quick Start](https://docs.nervos.org/docs/script/rust/rust-quick-start)
- Academy:
  - A4 [L1 Developer Training Course](https://nervos.gitbook.io/developer-training-course/lab-exercise-setup)
  - A5 CKB Script Programming Series
- Code:
   - [simple-lock](src/simple-lock/) example (a JavaScript hash lock on ckb-js-vm) on **devnet**
   - [ckb-rust-script](src/ckb-rust-script/), a workspace from [ckb-script-templates](https://github.com/cryptape/ckb-script-templates) with two contracts: `hello-world`, and `simple-lock`

## Key learnings

1. **A Script is a RISC-V program whose only output is an exit code.** CKB-VM runs the binary; `0` accepts the transaction and any other value rejects it, so the transaction is not committed, nothing changes and no fee is paid. Execution is deterministic: the same transaction always gives the same result, like a pure function. The off-chain code builds the new state (the outputs); the Scripts only answer yes or no. This is the "verify, not compute" idea from Week 2.

2. **A Script is `code_hash` + `hash_type` + `args`.** The code is found in a cell listed in `cell_deps`. `data`, `data1` and `data2` match the Blake2b hash of the binary: immutable, and pinned to CKB-VM v0, v1 or v2. `type` matches the Type Script hash of the code cell: upgradable, always the latest VM. `args` lets many owners share one binary (the default lock stores a public key hash there).

3. **A Lock Script runs when its cell is spent; a Type Script runs when its cell is spent or created.** Answer to the Week 5 self-check. Lock = who may spend, so output Locks never run; Type = which state changes are valid, so it runs for inputs and outputs (cells without a Type are ignored). A token rule such as `sum(inputs) >= sum(outputs)` needs the outputs checked; otherwise a transaction that only creates tokens would not run the Script. Proof in my tests: `test_output_lock_is_never_executed` puts a lock with invalid `args` (1 byte) on an output and the transaction still passes.

4. **Script groups.** Cells with the same Script (code, hash type and args) form one group and the Script runs **once** per group. `Source::GroupInput` and `GroupOutput` are virtual arrays of just that group; `Input` and `Output` are the whole transaction. For a Lock, `GroupOutput` is empty. `load_witness_args(0, GroupInput)` reads the witness at the position of the group's first input. In my tests:
   - One input and two inputs with the same lock both cost 33,572 cycles, and the second input has no witness at all, because the Script ran once and read only the first one.
   - Different `args` give different groups. Each group reads its own witness, so a wrong witness at index 1 fails with 11.
   - Consequence: a Script that reads only `GroupInput[0]` must be written knowing the other inputs of its group are not looked at.

5. **Syscalls and `ckb-std`.** A Script has no OS and reads the transaction through syscalls: an `ecall` instruction with the syscall number in register `a7` (93 `exit`, 2052 `load_script`, 2074 `load_witness`, 2177 `debug`). `ckb-std` wraps them in `syscalls` (raw) and `high_level` (`load_script`, `load_cell*`, `load_witness_args`, `load_tx_hash`, `QueryIter`). A source says where to read from: `Input`, `Output`, `CellDep`, `HeaderDep`, `GroupInput`, `GroupOutput`.

6. **Rust Script skeleton: `no_std`, `no_main`, allocator.**
   - `ckb_std::entry!(program_entry)` defines `_start` and the panic handler; `program_entry() -> i8` is the exit code.
   - `default_alloc!(16384, 1258306, 64)` sets a 16 KB fixed heap and a 1.2 MB dynamic heap, so `Vec` and `String` work through `alloc`.
   - The VM has 4 MB of memory, so always build with `--release`. The target is `riscv64imac-unknown-none-elf`; the Makefile adds `+zba,+zbb,+zbc,+zbs` and `lower-atomic`.
   - `make build` writes `build/release/<name>` (deploy this) and `<name>.debug` (about 19 MB, only for debugging).

7. **Exit codes are the contract's API.** `1`–`9` are the syscall errors (1 `INDEX_OUT_OF_BOUND`, 2 `ITEM_MISSING`, 3 `LENGTH_NOT_ENOUGH`, 4 encoding/invalid data), so custom codes start at 10. `simple-lock` returns 10 (no preimage), 11 (hash mismatch, the same code as the TypeScript lock in the tutorial) and 12 (`args` is not 32 bytes). A transaction with no witness returns 1: the syscall failed and `?` forwarded the error.

8. **Test with `ckb-testtool`, replay with `ckb-debugger`.**
   - `Context` mocks the chain: `deploy_cell` (binary to code cell), `build_script_with_hash_type`, `create_cell` (the input cells), `complete_tx` (adds the Script's `cell_deps`) and `verify_tx(&tx, max_cycles)`, which returns the cycles or an error whose text contains `error code N`; the tests assert on that.
   - The template helper `verify_and_dump_failed_tx` writes a failing transaction to `tests/failed_txs/*.json`; `ckb-debugger -f <file> -i 0 -t input -s lock` runs the lock of input 0 again and prints the Script log, the exit code and the cycles (`-i 1` for the second group).

9. **Cycles.**
   - Cost: 1 cycle per instruction, 3 for branches, 5 for multiply, 32 for divide, 500 + 0.25 per byte for a syscall, and 0.25 per byte of binary loaded (small Scripts are cheaper). The only hard limit is `max_block_cycles`, 3.5 billion on mainnet.
   - Measured: Hello World 7,182 cycles; Rust hash lock 33,572; the same hash lock in JavaScript (ckb-js-vm) 13,086,867 in the mock test and 13,084,116 on devnet. That is about **390 times** the Rust version, because QuickJS is interpreted inside CKB-VM. JavaScript is fine to learn and prototype; Rust is the choice for production.

10. **A hash lock proves knowledge of a secret, not ownership.** The preimage sits in `witness.lock`, public once the transaction is sent; I decoded `Hello World` from the witness hex on devnet. Anyone who sees the transaction first can copy it and build a competing one, and the tutorial returns the change to the same lock, so it is unsafe after the first use. The hash must be CKB's personalised Blake2b: use `ckb-hash` with `default-features = false, features = ["ckb-contract"]`, not a generic Blake2b.

11. **JavaScript lock layout and capacity.** With ckb-js-vm the lock Script is the interpreter (`code_hash`, `hash_type`) and `args` = 2 bytes + the `hash-lock.bc` code hash (32) + its hash type (1) + the expected hash (32) = 67 bytes, so the unlock transaction needs two `cell_deps`: the interpreter and the bytecode. The cell occupies 8 + 32 + 1 + 67 = 108 CKB, so 61 CKB is the minimum only for the default lock (see Week 2). An address encodes the whole lock including `args`, so each preimage gives a different address.

12. **Tooling notes from this week.**
    - The docs lag the template: the template uses edition 2024, `deploy_cell_by_name` and plain `capacity(1000)`; its generated `test_simple_lock` uses 1-byte `args`, which my lock rejects with 12, so I removed it as the docs suggest.
    - `ckb-std` 1.1 compiles a small C helper with clang; the template asks for clang 16 or newer. On Ubuntu 22.04 with clang 14 the build worked with `CLANG=clang` and `/usr/lib/llvm-14/bin` on `PATH` (for `llvm-ar`).
    - `cargo install ckb-debugger` stops without `protoc`; `offckb install ckb-debugger` downloads a prebuilt, checksum-verified binary instead.
    - pnpm 12 blocks dependency build scripts (`pnpm approve-builds`; I set esbuild and unrs-resolver to `true` and secp256k1 to `false`, which falls back to pure JavaScript).
    - `offckb deploy` asks for a Y/n confirmation; the tutorial's `pnpm run deploy` cannot pass `--yes`.

<!-- TODO(user): your own takeaways: what surprised you, what is still unclear, questions for Neon. -->

## Evidence

| # | Step | Command / action | Result | Evidence |
|---|------|------------------|--------|----------|
| 1 | Tool versions | `cargo generate --version`, `ckb-debugger --version`, `rustup target list --installed`, `offckb --version`, `pnpm --version` | cargo-generate 0.26.0; ckb-debugger 1.1.2 (installed with `offckb install ckb-debugger`, see learning 12); targets `x86_64-unknown-linux-gnu` and `riscv64imac-unknown-none-elf`; OffCKB 0.4.13; pnpm 12.3.4. | [tools-versions.png](imgs/tools-versions.png) |
| 2 | Create the workspace | `cargo generate gh:cryptape/ckb-script-templates workspace --name ckb-rust-script` | Project created with `Makefile`, `scripts/` and a `tests` crate; it has no contract yet. `cargo generate` also ran `git init`; I removed that `.git` to keep one repository. | [rust-generate-workspace.png](imgs/rust-generate-workspace.png) |
| 3 | Generate and build Hello World | `make generate CRATE=hello-world`, set `ckb_std::debug!("Hello World!");` in `program_entry`, then `make build` | Contract added under `contracts/hello-world` and to the workspace members. `build/release/hello-world` is 21,440 bytes; the `.debug` file is 19,784,672 bytes. | [rust-hello-build.png](imgs/rust-hello-build.png) |
| 4 | Run Hello World | `ckb-debugger --bin build/release/hello-world`, then `make test CARGO_ARGS="-- tests::test_hello_world --nocapture"` | Debugger: `Script log: Hello World!`, `Run result: 0`, `All cycles: 7182(7.0K)`. Test: `[contract debug] Hello World!`, `consume cycles: 7182`. | [rust-hello-run.png](imgs/rust-hello-run.png) |
| 5 | Build the Rust hash lock | `make build` (the contract was created with `make generate CRATE=simple-lock` and implemented in `contracts/simple-lock/src/main.rs`) | Both contracts built. `build/release/simple-lock` is 51,448 bytes. `cargo fmt --check` and `cargo clippy --all-targets` report nothing. | [rust-lock-build.png](imgs/rust-lock-build.png) |
| 6 | Test the Rust hash lock | `make test CARGO_ARGS="-- --nocapture --test-threads=1"` | 9 passed (Hello World + 8 for `simple-lock`). Correct preimage: 33,572 cycles. Wrong preimage: exit 11. Empty `lock` field: 10. No witness: 1. 31-byte `args`: 12. Output lock with invalid `args`: passes. Same-lock inputs: 33,572 cycles for 1 input and for 2. Different locks: pass, and fail with 11 when group B gets the wrong witness. | [rust-lock-tests.png](imgs/rust-lock-tests.png) |
| 7 | Replay a failed transaction | `ckb-debugger -f "$(ls -t tests/failed_txs/*.json \| head -1)" -i 0 -t input -s lock` after running only `test_wrong_preimage_is_rejected` | `Script log: simple-lock rejected the transaction: HashMismatch (11)`, `Run result: 11`, `All cycles: 35957(35.1K)`. The rejected run costs 2,385 cycles more than the accepted one (33,572); it also formats and prints the debug message. | [rust-lock-debugger.png](imgs/rust-lock-debugger.png) |
| 8 | Tutorial #5: build the JS contract | In `src/simple-lock`: `pnpm install`, then `pnpm build` | `Contract 'hash-lock' built successfully`; `dist/hash-lock.bc` is 27,810 bytes (the Rust lock is 51,448, but the JS one needs the ckb-js-vm interpreter on top). | [js-build.png](imgs/js-build.png) |
| 9 | Tutorial #5: start devnet and deploy | `offckb node` in one terminal; in another `pnpm run deploy -- --network devnet`, answer `Y` | `tx committed.` `Contract OutPoint: TODO(0x…:0)`; `ckb-js-vm OutPoint: TODO(0x…:15)`; both deployment files synchronized to `frontend/deployment/`. | [js-deploy.png](imgs/js-deploy.png) |
| 10 | Tutorial #5: frontend and hash-lock address | In `src/simple-lock/frontend`: `pnpm dev`, open http://localhost:3000, keep the preimage `Hello World` | Deployment health `DEVNET` / `READY`. Contract code hash `0xcd262cb3…d23519`; blake2b-256 of `Hello World` is `0x106911e4…59ca`. Address `TODO(ckt1…)`; lock `args` `0x0000` + code hash + hash type + `0x106911e4…59ca`. | [js-ui-ready.png](imgs/js-ui-ready.png) |
| 11 | Tutorial #5: deposit | `offckb deposit --network devnet <hash-lock address> 300` | Deposit tx hash `TODO(0x…)`; after it was committed, Refresh in the UI showed Total live capacity 300 CKB. | [js-deposit.png](imgs/js-deposit.png), [js-ui-funded.png](imgs/js-ui-funded.png) |
| 12 | Tutorial #5: wrong preimage | Preimage to reveal `wrong secret`, Transfer CKB | Status `failed`: "The preimage does not match this hash lock (error 11). The transaction was rejected atomically and the original cells remain live." Capacity stays 300 CKB. | [js-ui-wrong.png](imgs/js-ui-wrong.png) |
| 13 | Tutorial #5: correct preimage | Preimage to reveal `Hello World`, send 99 CKB to the default receiver, Transfer CKB | Status `pending` then `committed`; tx hash `TODO(0x…)`. | [js-ui-unlock.png](imgs/js-ui-unlock.png) |
| 14 | Tutorial #5: inspect the unlock transaction | `curl` `get_transaction` with the hash from #13 on `http://127.0.0.1:8114` | `committed`; `cycles` `0xc7a5d4` (13,084,116); 1 input (the deposit cell from #11); outputs: #0 99 CKB with the default lock, #1 200.99999506 CKB back to the hash lock; fee 494 shannons (300 − 99 − 200.99999506); 2 `cell_deps` (`hash-lock.bc` and ckb-js-vm); `witnesses[0]` is `WitnessArgs` whose `lock` is `0x48656c6c6f20576f726c64`, the text `Hello World`. | [js-rpc-unlock-tx.png](imgs/js-rpc-unlock-tx.png) |
| 15 | Tutorial #5: tests | `pnpm test` | 2 suites, 12 tests passed; the mock test logs `Run result: 0` and `All cycles: 13086867(12.5M)`. | [js-tests.png](imgs/js-tests.png) |
| 16 | Academy A4 | academy.ckb.dev, course A4 | TODO(lessons completed, score if shown) | [academy-a4.png](imgs/academy-a4.png) |
| 17 | Academy A5 | academy.ckb.dev, course A5 | TODO(lessons completed, score if shown) | [academy-a5.png](imgs/academy-a5.png) |
