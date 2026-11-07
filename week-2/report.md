# Week 2 — 2026-11-08

## Completed

- [How CKB works](https://docs.nervos.org/docs/getting-started/how-ckb-works)
- [Cell](https://docs.nervos.org/docs/tech-explanation/cell)
- [Script](https://docs.nervos.org/docs/tech-explanation/script)
- [Transaction](https://docs.nervos.org/docs/tech-explanation/transaction)
- [A1 — CKB basic theoretical knowledge](https://academy.ckb.dev/courses/basic-theory)
- [A2 — CKB basic practical operation](https://academy.ckb.dev/courses/basic-operation)
- Notes: [note.md](note.md)

## Key learnings

1. **Cells are immutable state.** A Cell combines capacity, a lock Script, an optional type Script, and data. A Cell cannot be edited in place: a transaction consumes it and creates a replacement Cell with the desired state.

   | Field | Purpose |
   |---|---|
   | `capacity` | Amount of CKBytes assigned to the Cell; it also covers the occupied storage. |
   | `lock` | Required Script that controls whether the Cell can be spent. |
   | `type` | Optional Script that checks rules for the Cell's creation or transition. |
   | `data` | State stored in the Cell. In a transaction, output data is carried separately in `outputs_data`. |

   An `OutPoint` identifies a particular output by its transaction hash and output index. A Cell that has not been consumed is a **live Cell** and can be referenced by a later transaction.

2. **Capacity is both value and storage.** CKBytes represent the capacity available to store on-chain data. The minimum depends on the Cell's occupied fields; a Cell with the standard secp256k1 lock and no data requires 61 CKB (61 CKBytes).

3. **Scripts separate authorization from state rules.**

   | Script | Required? | Role |
   |---|---|---|
   | Lock Script | Yes | Authorizes spending an input Cell, for example by verifying a signature. |
   | Type Script | No | Enforces application rules on Cells, such as validating a token supply or a state transition. |

   A Script is identified by `code_hash`, `hash_type`, and `args`. Its binary is referenced through a dependency Cell in `cell_deps`; `hash_type` tells CKB how to resolve the code hash. CKB groups inputs with the same Lock Script so that the grouped Script is executed once rather than once per input.

4. **Transactions consume inputs and create outputs.** The transaction describes the state transition; it does not edit existing Cells.

   | Field | What it contains |
   |---|---|
   | `version` | Transaction format/version information. |
   | `cell_deps` | OutPoints for dependency Cells, such as Cells containing Script code. |
   | `header_deps` | Block-header hashes needed by the transaction. |
   | `inputs` | References to the Cells being consumed. |
   | `outputs` | New Cell definitions, including capacity and Scripts. |
   | `outputs_data` | Data for the corresponding output Cells. |
   | `witnesses` | Data supplied for Script validation, such as a signature. |

   The arrays of `outputs` and `outputs_data` correspond by index. For a capacity transfer, total input capacity must cover output capacity; the remainder is the transaction fee.

5. **Transaction flow, from construction to confirmation:**

   ```mermaid
   flowchart LR
     A[Choose live input Cells] --> B[Build outputs and output data]
     B --> C[Add cell and header dependencies]
     C --> D[Provide witnesses and sign]
     D --> E[Broadcast transaction]
     E --> F[Nodes validate inputs, capacity, and Scripts]
     F --> G[Valid transaction enters the mempool]
     G --> H[Miner includes it in a block]
     H --> I[Inputs are consumed; outputs become live Cells]
   ```

   **Cell-level view:**

   ```text
   inputs (OutPoints to live Cells)
       + cell_deps (Script code) + header_deps + witnesses
       -> transaction validation
       -> outputs (new Cells) + outputs_data
       -> fee = total input capacity - total output capacity
   ```

6. **CKB emphasizes verification, not on-chain computation.** The transaction is assembled off-chain. On-chain Scripts validate that the proposed transition satisfies authorization and application rules.

7. **Built-in Script code is kept available.** CKB's built-in Script code Cells use a lock that cannot be unlocked, so those dependency Cells cannot be spent and their code remains available.

## Evidence

| # | Step | Result | Evidence |
|---|---|---|---|
| 1 | Record Cell, Script, and Transaction concepts | Notes cover the structures, Script roles, and transaction flow | [note.md](note.md) |
| 2 | Complete Academy A1 and A2 | A1 and A2 completed; screenshots were not included in the materials available for this report | — |
| 3 | Hand-drawn transaction diagram | Not included in the materials available for this report | — |
