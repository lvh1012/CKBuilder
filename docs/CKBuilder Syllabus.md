# CKBuilder Syllabus — 12 Weeks, from Zero to Capstone

> **Learner:** Software Engineer (fullstack, microservices, cloud) with no blockchain experience.
> **Requirements source:** [CKB Builder Handbook](CKB%20Builder%20Handbook.md), [CKBuilder T&Cs](CKBuilder%20T&Cs.md).
> **Time commitment:** the programme requires at least 4–5h/week. This syllabus targets ~7–8h/week: **Core** (~5h, enough to meet the requirements) + **Stretch** (~2–3h, if time allows).

## Chosen stack

| Layer | Tooling | Why |
|---|---|---|
| Off-chain / dApp | TypeScript + [CCC](https://docs.ckbccc.com/docs/CCC) | Builds on fullstack experience; recommended by the Handbook for beginners |
| On-chain Script | Rust + [ckb-script-templates](https://github.com/cryptape/ckb-script-templates) + `ckb-std` | The Handbook recommends Rust for smart contracts |
| Local devnet / deploy | [OffCKB](https://docs.nervos.org/docs/sdk-and-devtool/offckb) | The official replacement for Capsule |
| Skip | Lumos, Capsule | Deprecated — only read for concepts when older material mentions them |

---

## 1. Required deliverables and target weeks

| Requirement | Source | Done by |
|---|---|---|
| Weekly report on GitHub (once a week, fixed day, with screenshots) | T&Cs §1.2, Handbook | **Every week** |
| CKB Academy — 8 modules + interactive tutorial | T&Cs §1.2 | Weeks 1 → 8 |
| CCC Playground — complete the examples | T&Cs §1.2 | Week 4 |
| Set up your own CKB node + dev environment | T&Cs §1.2 | Week 1 (devnet), Week 3 (testnet node) |
| 5 Beginner tutorials (Transfer, Store Data, Token, DOB, Simple Lock) | Handbook — Beginner | Weeks 3 → 8 |
| Capstone: a small application | T&Cs §1.2 (month 3) | Weeks 9 → 12 |

### Schedule for the 8 [CKB Academy](https://academy.ckb.dev/courses) modules

| # | Module | Week | Notes |
|---|---|---|---|
| A1 | CKB basic theoretical knowledge | 1–2 | Theoretical foundation |
| A2 | CKB basic practical operation | 2 | Interactive — build a transaction by hand and send it to testnet |
| A3 | Construct and Send Your First CKB Transaction | 3 | |
| A4 | L1 Developer Training Course | 5–7 | Labs use Lumos → read for concepts, redo them with CCC/Rust |
| A5 | CKB Script Programming Series | 5–7 | Same content as the Script course on docs.nervos.org → study them together |
| A6 | Dapps with CKB Workshop | 7 | Video; watch the Capsule/Lumos parts for concepts only |
| A7 | Getting Started With NFTs | 8 | Study alongside Spore/DOB |
| A8 | Start Your CKB Development Journey | 8 | Hackathon recap → source of Capstone ideas |
| + | Academy **Playground** | 2 | The "interactive tutorial" |

---

## 2. Mental model: mapping Web2 to CKB

Read this table before Week 2, then come back to it whenever a new concept appears.

| Web2 / Backend | CKB | Notes |
|---|---|---|
| An immutable record | **Cell** = `capacity` + `lock` + `type` + `data` | No in-place updates: consume old cells, create new ones |
| Event sourcing / state transition | **Transaction**: `inputs` → `outputs` | A generalised UTXO model (Bitcoin UTXO + data + script) |
| AuthN/AuthZ middleware | **Lock Script** | "Who is allowed to spend this cell?" |
| Schema validation / DB constraint / domain invariant | **Type Script** | "Is this state transition valid?" |
| Server computes, then stores | **Off-chain compute, on-chain verify** | The client builds the whole transaction; Scripts only check it is valid |
| Storage billing (S3) | 1 CKByte = 1 byte of on-chain state | Minimum cell is 61 CKB = 8 (capacity) + 32 (code_hash) + 1 (hash_type) + 20 (args) |
| A deployed binary/container | A Script binary lives in a cell and is referenced via `cell_deps` + `code_hash` | |
| CPU quota / timeout | **Cycles** | Execution cost limit in CKB-VM (RISC-V) |
| Protobuf | **Molecule** | CKB's standard serialization format |
| Unit tests with mocks | `ckb-testtool` (mock transactions) + `ckb-debugger` | |

---

## 3. Weekly plan

Each week has: **Goal → Study → Practice → Evidence for the report → Self-check**.
The **Rust track** line is only for those who don't know Rust yet; skip it if you do.

### MONTH 1 — Blockchain fundamentals, Cell Model, first dApp

**End-of-month outcome:** understand blockchain basics and the Cell Model, have a devnet + testnet node, send transactions, and write a TypeScript dApp with CCC.

#### Week 1 — Blockchain 101 and environment setup

- **Goal:** understand what problem blockchain solves; have a dev log repo and a local devnet running.
- **Study (Core):**
  - Core concepts: hash functions, public/private keys, digital signatures, blocks, chains, Proof of Work, the UTXO model vs the Account model.
  - [Bitcoin whitepaper](https://bitcoin.org/bitcoin.pdf) (9 pages; not in the Handbook, but the foundation for the Cell Model).
  - [Nervos Nation videos](https://www.youtube.com/c/NervosNation): the videos on UTXO/PoW and the Cell Model.
  - [Introduction to Nervos CKB](https://docs.nervos.org/docs/ckb-fundamentals/nervos-blockchain).
  - **A1** — first half.
- **Practice:**
  - Create a public CKBuilder repo on GitHub with a `reports/` folder. **Pick a fixed day of the week for submitting reports.**
  - Windows: install **WSL2 (Ubuntu)** and do all builds there, since Rust scripts need `clang`/`make` and the toolchain breaks easily on native Windows.
  - Install Node.js LTS, Rust (`rustup`), OffCKB (`npm i -g @offckb/cli`), run `offckb node`, and list the built-in accounts with `offckb accounts`.
  - (Stretch) Connect [CKB AI MCP](https://ckb-ai.ckbdev.com/) to your AI assistant for CKB context while coding.
- **Rust track:** [The Rust Book](https://doc.rust-lang.org/book/) chapters 1–4 (ownership, borrowing).
- **Evidence:** screenshot of the running devnet; a short write-up in your own words comparing UTXO and Account. Commit `4a4539a` (hello-world deployed to devnet/testnet) is also good evidence.
- **Self-check:** Why are digital signatures needed? What is a double-spend and how does PoW prevent it? How does UTXO differ from Account?

#### Week 2 — The Cell Model, the heart of CKB

- **Goal:** read and understand the structure of Cells, Scripts and Transactions.
- **Study (Core):**
  - [How CKB works](https://docs.nervos.org/docs/getting-started/how-ckb-works).
  - **A1** — the rest; **A2** (interactive).
  - Concepts to master: Cell (`capacity`, `lock`, `type`, `data`), `OutPoint`, live vs dead cells, Script (`code_hash`, `hash_type`, `args`), Transaction structure (`inputs`, `outputs`, `outputs_data`, `cell_deps`, `header_deps`, `witnesses`), transaction fees.
- **Study (Stretch):** [CKB: a Blockchain Developer's Dream](https://www.nervos.org/knowledge-base/ckb_blockchain_developers_dream), [Tokenomics of Nervos](https://www.nervos.org/knowledge-base/tokenomics_of_nervos_network).
- **Practice:**
  - Academy **Playground** (interactive tutorial).
  - Create a testnet wallet, get CKB from the [Faucet](https://faucet.nervos.org/), and look up addresses and transactions on the [Testnet Explorer](https://pudge.explorer.nervos.org/).
  - Draw your own diagram of a transaction (inputs → outputs, cell_deps, witnesses) in Excalidraw or draw.io.
- **Rust track:** chapters 5–6 (structs, enums, `match`).
- **Evidence:** screenshots of A1/A2 completion, your transaction diagram.
- **Self-check:** Why does a cell need at least 61 CKB? How do Lock Scripts and Type Scripts differ? Why is CKB "verify, not compute"?

#### Week 3 — First transaction and your own CKB node

- **Goal:** run your own node, interact through RPC, send transactions from code.
- **Study (Core):**
  - **A3**.
  - [Quick start](https://docs.nervos.org/docs/getting-started/quick-start): networks (devnet, testnet, mainnet) and RPC.
  - [CKB-CLI](https://docs.nervos.org/docs/sdk-and-devtool/ckb-cli).
- **Practice:**
  - **Run a testnet node** (required deliverable): a full node (`ckb init --chain testnet`, then `ckb run`) if you have the disk space and time to sync; otherwise use the [CKB light client](https://github.com/nervosnetwork/ckb-light-client).
  - Call RPC by hand with curl or Postman: `get_tip_block_number`, `get_transaction`, `get_live_cell`.
  - Tutorial #1: [Transfer CKB](https://docs.nervos.org/docs/dapp/transfer-ckb).
  - `ckb-cli`: create an account, check the balance, transfer CKB on testnet.
- **Rust track:** chapters 8–9 (collections, error handling with `Result`).
- **Evidence:** screenshot of the node syncing or synced, tx hash on the Explorer.
- **Self-check:** Open any transaction on the Explorer and explain every field.

#### Week 4 — CCC and TypeScript dApps

- **Goal:** write a complete off-chain dApp with CCC.
- **Study (Core):** [CCC docs](https://docs.ckbccc.com/docs/CCC), [CCC App](https://docs.ckbccc.com/docs/ccc-app), [Code examples](https://docs.ckbccc.com/docs/code-examples), [API reference](https://api.ckbccc.com/). Concepts: `Client`, `Signer`, building a transaction, completing inputs by capacity and calculating the fee.
- **Practice:**
  - **[CCC Playground](https://docs.ckbccc.com/docs/playground): complete all examples** (required deliverable).
  - Tutorial #2: [Store Data on Cell](https://docs.nervos.org/docs/dapp/store-data-on-cell).
  - Tutorial #3: [Create Fungible Token](https://docs.nervos.org/docs/dapp/create-token).
  - (Stretch) A small Next.js app: connect a wallet, show the balance, transfer CKB.
- **Rust track:** chapter 10 (generics, traits, lifetimes).
- **Evidence:** Playground screenshots, code links, tx hash of the token you created.

**Month 1 review checklist**

- [ ] 4 weekly reports, submitted on the fixed day
- [ ] Academy A1, A2, A3 + Playground
- [ ] Devnet (OffCKB) + testnet node
- [ ] CCC Playground examples
- [ ] Tutorials: Transfer CKB, Store Data on Cell, Create Fungible Token

---

### MONTH 2 — On-chain Scripts in Rust, token standards, Capstone decision

**End-of-month outcome:** able to write, test, debug and deploy Scripts in Rust; understand UDT, Molecule, Type ID and Spore; a Capstone proposal discussed with Neon.

#### Week 5 — Script 101: your first Lock Script

- **Goal:** understand how Scripts run in CKB-VM; write and test your own Lock Script.
- **Study (Core):**
  - [Introduction to Script](https://docs.nervos.org/docs/script/intro-to-script).
  - Script course [Class 1: Validation Model](https://docs.nervos.org/docs/script-course/intro-to-script-1), [Class 2: Script Basics](https://docs.nervos.org/docs/script-course/intro-to-script-2) (also part of **A5**).
  - [Rust quick start for Scripts](https://docs.nervos.org/docs/script/rust/rust-quick-start).
  - **A4** — the lessons on the Cell Model and transactions.
  - Concepts: syscalls (`load_script`, `load_cell`, `load_witness`), `Source::GroupInput`, Script groups, `no_std` + `alloc`, error codes (a non-zero exit code means the transaction is rejected), cycles.
- **Practice:**
  - Create a workspace from ckb-script-templates, generate a new contract, build it and run the tests.
  - Tutorial #5: [Build a Simple Lock](https://docs.nervos.org/docs/dapp/simple-lock).
- **Evidence:** passing tests, a tx using the Simple Lock on devnet.
- **Self-check:** When does a Lock Script run: when a cell is created or when it is spent? When does a Type Script run?

#### Week 6 — Tokens and serialization

- **Goal:** understand CKB's token standards and Molecule.
- **Study (Core):**
  - [Class 3: UDT](https://docs.nervos.org/docs/script-course/intro-to-script-3).
  - sUDT: [RFC 0025](https://docs-xi-two.vercel.app/docs/rfcs/0025-simple-udt/0025-simple-udt), [tutorial](https://github.com/nervosnetwork/ckb-cli/wiki/UDT-%28sudt%29-Operations-Tutorial).
  - xUDT: [Introduction](https://docs.nervos.org/docs/tech-explanation/xudt), [RFC 0052](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0052-extensible-udt/0052-extensible-udt.md).
  - [Molecule and serialization](https://docs.nervos.org/docs/serialization/serialization-molecule-in-ckb).
  - **A4** — the token lessons: read for concepts, redo them with CCC instead of Lumos.
- **Practice:**
  - Rewrite an sUDT-like Type Script in Rust: owner mode (mint), the `sum(inputs) >= sum(outputs)` rule, with tests for both valid cases and attacks (unauthorised mint, inflating supply).
  - Define a small Molecule schema and generate Rust/TS code from it.
- **Evidence:** code links and test results.

#### Week 7 — Testing, debugging, Type ID, Nervos DAO

- **Goal:** a professional Script workflow: test, debug, deploy, upgrade.
- **Study (Core):**
  - [Class 5: Debugging](https://docs.nervos.org/docs/script-course/intro-to-script-5), [Class 6: Type ID](https://docs.nervos.org/docs/script-course/intro-to-script-6).
  - [CKB Debugger](https://github.com/nervosnetwork/ckb-standalone-debugger).
  - **A4** — the rest; **A5** — the rest; **A6** (video).
- **Study (Stretch):** Nervos DAO: [RFC 0023](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0023-dao-deposit-withdraw/0023-dao-deposit-withdraw.md), [NervDAO](https://github.com/ckb-devrel/nervdao).
- **Practice:**
  - Debug your Week 6 Script with `ckb-debugger`; measure cycles.
  - Deploy the Script to testnet with OffCKB, using Type ID so the Script can be upgraded.
  - Call the deployed Script from a CCC frontend (end-to-end).
  - (Stretch) Deposit to and withdraw from Nervos DAO on testnet with CCC.
- **Evidence:** deploy tx on the Explorer, debugger screenshot.
- **Self-check:** How do `hash_type` values `data1` and `type` differ when upgrading a Script?

#### Week 8 — Spore/DOB and the Capstone proposal

- **Goal:** finish the Academy and Beginner material; settle on a Capstone idea.
- **Study (Core):**
  - **A7** (NFTs), **A8** (Start Your CKB Development Journey).
  - [Spore protocol introduction](https://docs.nervos.org/docs/tech-explanation/spore-protocol), [Spore docs](https://docs.spore.pro/).
  - [2026 opportunity map](https://talk.nervos.org/t/ai-machine-payments-and-fiber-in-2026-an-opportunity-map-for-ckb-and-fiber-developers/10665).
  - (Stretch) Skim Classes 4, 7–10 (WASM, Duktape, language choices); not required on the Rust path.
- **Practice:**
  - Tutorial #4: [Create DOB](https://docs.nervos.org/docs/dapp/create-dob).
  - **Write a one-page Capstone proposal:** problem, architecture, cell design, MVP scope. **Send it to Neon/DevRel for feedback.**
- **Evidence:** DOB minted on testnet; link to the proposal.

**Month 2 review checklist**

- [ ] 4 weekly reports
- [ ] Academy A4, A5, A6, A7, A8 → **all 8/8 modules**
- [ ] Tutorials: Simple Lock, Create DOB → **all 5/5 Beginner tutorials**
- [ ] At least one Rust Script with tests, deployed to testnet
- [ ] Capstone proposal sent to Neon

---

### MONTH 3 — Capstone and advanced topics

**End-of-month outcome:** a small application running on testnet, with a README, architecture diagram and demo video.

**Scope rule:** the MVP must run end-to-end before the end of Week 11; Week 12 is for polish and showcase only, no new features.

#### Week 9 — Capstone design and skeleton

- **Practice (Core):**
  - Design: split on-chain (Scripts) / off-chain (services, indexer) / frontend; cell layout (Molecule schema); choice of Lock/Type; diagrams of the transaction flows (create, update, destroy).
  - Set up the repo: `contracts/` (Rust), `app/` (Next.js + CCC), `tests/`; GitHub Actions running build + tests for the Scripts.
- **Study (Advanced, pick what fits your Capstone):** [Fiber docs](https://www.fiber.world/docs) · [SSRI](https://talk.nervos.org/t/en-cn-script-sourced-rich-information-script/8256/2) · [xUDT extensions](https://docs.nervos.org/docs/ecosystem-scripts/xudt#how-xudt-works) · [Ecosystem scripts](https://docs.nervos.org/docs/ecosystem-scripts/introduction).
- **Evidence:** architecture diagram, green CI.

#### Week 10 — Build on-chain

- **Practice (Core):** implement the Scripts; test both happy paths and attack cases (forged inputs, capacity theft, unauthorised data changes); deploy to testnet.
- **Study (Stretch):** [Pausable UDT](https://github.com/Alive24/pausable-udt) and its [audit report](https://github.com/Alive24/pausable-udt/blob/master/20241224-Pausable-UDT-Final-Audit-Report.pdf), to learn how a production contract gets reviewed.
- **Evidence:** test report, deploy tx.

#### Week 11 — Build off-chain and integrate

- **Practice (Core):** frontend/backend using CCC, wallet connection, end-to-end run on testnet; deploy the frontend (Vercel or a cloud you know).
- **Study (Stretch):** [RGB++](https://rgbpp.com/docs/introduction), [iCKB](https://www.nervos.org/knowledge-base/Unlocking_CKB_Liquidity_iCKB): read to broaden your perspective.
- **Evidence:** link to the running app, end-to-end tx hashes.

#### Week 12 — Polish and showcase

- **Practice (Core):** README (problem, architecture, how to run, Script addresses on testnet), a 3–5 minute demo video, a final report with a 3-month retrospective.
- **(Stretch):** post an introduction on [Nervos Talk](https://talk.nervos.org/); if the idea is strong, consider a [Spark grant](https://talk.nervos.org/t/ckb-eco-fund-spark-program-mini-grant-initiative/8752).

**Month 3 review checklist**

- [ ] 4 weekly reports
- [ ] Capstone running on testnet, with README, diagram and demo video
- [ ] Capstone presented or sent to Neon

---

## 4. Capstone ideas that fit your background

| Idea | Skills used | Difficulty |
|---|---|---|
| **Token Vesting Vault:** a Rust Lock Script using `since` to lock CKB/xUDT until a given time, with a CCC dashboard | Lock Script, `since`, xUDT, CCC | Medium. **Recommended as a safe baseline** |
| **Pay-per-request API Gateway:** middleware that requires payment via a Fiber invoice before returning a response (similar to HTTP 402) | Fiber, microservices, cloud | High. Matches the 2026 opportunity map, stands out more, better grant prospects |
| **DOB Minting Platform:** mint Spore DOBs by cluster, with a decoder for display | Spore SDK, CCC, frontend | Low to medium |

---

## 5. Weekly report template

The Handbook has an [example report](https://github.com/JouahriAli/ckb-builder-track/blob/main/ckb_weekly_report_w1.md). Suggested outline:

```markdown
# Week N — YYYY-MM-DD

## Completed
- Academy: ...
- Tutorials / Code: ... (commit links, tx hashes on the Explorer)

## Key learnings

## Challenges and how I solved them

## Evidence
(screenshots)

## Plan for next week
```

> **Note:** the T&Cs require reports to be original. AI may be used to tidy up wording, but not to write the report. A report covering several weeks counts as a single report, and the stipend is based on the number of reports in the month.

---

## 6. Supplementary resources

- Community: [Learning CKB — 24 lessons](https://website-sooty-chi-72.vercel.app/lessons), [Learn CKB in 45 mins](https://github.com/truthixify/learn-ckb-in-45-minutes) — good for review after Week 2.
- [Nervos RFCs](https://github.com/nervosnetwork/rfcs) — look up standards when you need the details.
- The Handbook's [reading list](CKB%20Builder%20Handbook.md#reading-list-understanding-ckb) — read gradually, one article a week.
- Stuck for more than a day: ask Neon or CKB DevRel early. The T&Cs require you to report blockers promptly.
