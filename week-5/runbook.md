# Runbook Tuần 5 — chụp 18 ảnh evidence cho `report.md`

Mục tiêu: sau khi làm hết runbook này, thư mục `week-5/imgs/` có đủ 18 ảnh đúng tên mà `report.md` đang link, và mọi `TODO(...)` trong báo cáo được thay bằng giá trị thật của bạn.

Thời gian ước tính: Phần A (Rust) ~40 phút, Phần B (tutorial #5) ~30 phút, Phần C (Academy) ~60–90 phút, Phần D (hoàn thiện) ~15 phút.

> **Về nguồn gốc các con số.** Số liệu trong `report.md` (cycles, kích thước file, mã lỗi…) lấy từ một lần chạy thử trên chính máy này. Khi chạy lại, hãy đối chiếu từng con số với ảnh của bạn; nếu khác, **sửa báo cáo theo ảnh của bạn**. Chỉ các giá trị đặc thù từng lần chạy (tx hash, OutPoint, địa chỉ) được để `TODO(...)`.
>
> **Về code.** `src/ckb-rust-script/contracts/simple-lock` và `tests/src/test_simple_lock.rs` do Claude soạn dựa trên tài liệu "Example: Simple Lock". Hãy đọc từng dòng, chạy lại và sửa theo ý bạn trước khi nộp.

## Danh sách ảnh

> **Cập nhật 2026-11-24.** `report.md` hiện chỉ giữ 12 dòng Evidence có ảnh. Các dòng cho ảnh chưa chụp (workspace, preimage sai, `pnpm test`, Academy A4/A5) đã bị xóa; cột "Dòng Evidence" dưới đây dùng số dòng mới (— nghĩa là hiện chưa có dòng nào). Nếu chụp thêm ảnh, thêm một dòng mới ở cuối bảng Evidence.

| Ảnh | File (trong `week-5/imgs/`) | Bước | Dòng Evidence | Chụp gì |
|---|---|---|---|---|
| 1 | `tools-versions.png` | A1 | #1 | Version các công cụ |
| 2 | `rust-generate-workspace.png` | A2 | — | `cargo generate` tạo workspace |
| 3 | `rust-hello-build.png` | A3 | #2 | Sinh contract và `make build` |
| 4 | `rust-hello-run.png` | A4 | #3 | Chạy Hello World bằng debugger và test |
| 5 | `rust-lock-build.png` | A5 | #4 | Build `simple-lock` (Rust) |
| 6 | `rust-lock-tests.png` | A6 | #5 | 9 test pass |
| 7 | `rust-lock-debugger.png` | A7 | #6 | Phát lại giao dịch lỗi bằng `ckb-debugger` |
| 8 | `js-build.png` | B1 | #7 | Build contract TypeScript |
| 9 | `js-deploy.png` | B2 | #8 | Deploy lên devnet |
| 10 | `js-ui-ready.png` | B3 | #9 | Frontend: deployment READY + địa chỉ hash lock |
| 11 | `js-deposit.png` | B4 | #10 | `offckb deposit` |
| 12 | `js-ui-funded.png` | B4 | #10 | UI hiện 300 CKB |
| 13 | `js-ui-wrong.png` | B5 | — | Preimage sai → lỗi 11 |
| 14 | `js-ui-unlock.png` | B6 | #11 | Preimage đúng → committed |
| 15 | `js-rpc-unlock-tx.png` | B7 | #12 | `get_transaction` của giao dịch unlock |
| 16 | `js-tests.png` | B8 | — | `pnpm test` |
| 17 | `academy-a4.png` | C | — | Tiến độ Academy A4 |
| 18 | `academy-a5.png` | C | — | Tiến độ Academy A5 |

## 0. Chuẩn bị chung

### 0.1 Quy ước chụp ảnh

- Dùng `PrtSc` của Ubuntu → **Selection** → chọn vùng. Ảnh nằm ở `~/Pictures/Screenshots/`.
- Mỗi ảnh terminal phải thấy **cả lệnh lẫn kết quả**. Tăng cỡ chữ terminal (`Ctrl` + `+`) để đọc được, cắt bỏ phần thừa.
- Không để lộ private key. Các lệnh trong runbook này chỉ dùng key devnet mặc định của OffCKB, không có key testnet.
- Thêm hàm này vào terminal để đổi tên và chuyển ảnh mới nhất vào đúng chỗ:

```bash
shot() {
  mv "$(ls -t ~/Pictures/Screenshots/*.png | head -1)" ~/Documents/CKBuilder/week-5/imgs/"$1".png \
    && ls -l ~/Documents/CKBuilder/week-5/imgs/"$1".png
}
# Chụp xong ảnh đầu tiên:  shot tools-versions
```

### 0.2 Nhánh git và thư mục

```bash
cd ~/Documents/CKBuilder
git branch --show-current        # phải là week-5
mkdir -p week-5/imgs
```

### 0.3 Biến môi trường clang (mọi terminal chạy `make`)

Template `ckb-script-templates` yêu cầu clang ≥ 16, máy này chỉ có clang 14 (Ubuntu 22.04). Khi thử, bản 14 vẫn build được hai contract này nếu chỉ rõ `CLANG` và đưa công cụ LLVM 14 vào `PATH` (thiếu `llvm-ar` nếu không có dòng thứ hai):

```bash
export CLANG=clang
export PATH=/usr/lib/llvm-14/bin:$PATH
```

Cách "chuẩn" thay thế (cần `sudo`): cài clang 18+ theo `apt.llvm.org` rồi bỏ hai dòng trên.

### 0.4 Công cụ đã cài sẵn

Khi soạn bản nháp này, Claude đã cài trên máy bạn: `cargo-generate` (`cargo install cargo-generate`), target `riscv64imac-unknown-none-elf` (`rustup target add …`) và `ckb-debugger` 1.1.2 (`offckb install ckb-debugger`, tạo shim trong `~/.nvm/versions/node/v26.13.0/bin/`). Nếu thiếu, cài lại bằng đúng các lệnh đó. `cargo install ckb-debugger` sẽ lỗi vì máy chưa có `protoc`; đó là lý do dùng `offckb install`.

### 0.5 Bố trí terminal

| Terminal | Dùng cho |
|---|---|
| T1 | Phần A (Rust) |
| T2 | `offckb node` (Phần B, để chạy suốt) |
| T3 | Lệnh của tutorial #5, trong `week-5/src/simple-lock` |
| T4 | Frontend `pnpm dev` |

---

## Phần A — Rust Script (ảnh 1–7)

### A1. Version công cụ → `tools-versions.png`

```bash
cargo generate --version
ckb-debugger --version
rustup target list --installed
offckb --version
pnpm --version
```

Kỳ vọng: `cargo-generate 0.26.0`, `ckb-debugger 1.1.2`, hai target (`riscv64imac-unknown-none-elf` và `x86_64-unknown-linux-gnu`), `0.4.13`, `12.3.4`.

### A2. Tạo workspace → `rust-generate-workspace.png`

Làm trong thư mục tạm để chụp lại đúng quy trình từ đầu (bản có code hoàn chỉnh đã nằm sẵn trong repo ở `week-5/src/ckb-rust-script`):

```bash
mkdir -p ~/ckb-demo && cd ~/ckb-demo
cargo generate gh:cryptape/ckb-script-templates workspace --name ckb-rust-script
cd ckb-rust-script
find . -not -path './.git/*' -type f | sort
```

Ảnh phải thấy: dòng `Done! New project created …`, và danh sách file có `Makefile`, `scripts/find_clang`, `tests/Cargo.toml`, `tests/src/lib.rs`, `tests/src/tests.rs` (chưa có thư mục `contracts/`).

### A3. Sinh contract và build → `rust-hello-build.png`

Vẫn trong `~/ckb-demo/ckb-rust-script`, T1 đã `export` biến ở mục 0.3:

```bash
make generate CRATE=hello-world
```

Mở `contracts/hello-world/src/main.rs` và đổi dòng `ckb_std::debug!("This is a sample contract!");` thành `ckb_std::debug!("Hello World!");`. Rồi:

```bash
make build
ls -l build/release
```

Lần đầu mất 1–2 phút (tải và biên dịch `ckb-std`). Ảnh phải thấy: `make generate` thành công, `Finished release …`, `Copying binary hello-world to build directory`, và `ls -l` có `hello-world` (≈ 21,4 KB, kỳ vọng 21440 byte) cùng `hello-world.debug` (≈ 19,8 MB).

### A4. Chạy Hello World → `rust-hello-run.png`

```bash
ckb-debugger --bin build/release/hello-world
make test CARGO_ARGS="-- tests::test_hello_world --nocapture"
```

Lần đầu `make test` biên dịch `ckb-testtool` mất vài phút. Kỳ vọng: debugger in `Script log: Hello World!`, `Run result: 0`, `All cycles: 7182(7.0K)`; test in `[contract debug] Hello World!` và `consume cycles: 7182`, `test tests::test_hello_world ... ok`. Chụp cả hai kết quả trong một ảnh (cuộn terminal nếu cần).

### A5. Build `simple-lock` → `rust-lock-build.png`

Chuyển sang bản trong repo (đã có `hello-world` và `simple-lock`):

```bash
cd ~/Documents/CKBuilder/week-5/src/ckb-rust-script
make build
ls -l build/release
cargo fmt --all -- --check && echo fmt-ok
```

Ảnh phải thấy: cả hai contract được build, `simple-lock` ≈ 51,4 KB (kỳ vọng 51448 byte), và `fmt-ok`. (Chạy thêm `cargo clippy --all-targets` nếu muốn; kỳ vọng không có cảnh báo.)

### A6. Test → `rust-lock-tests.png`

```bash
rm -rf tests/failed_txs
make test CARGO_ARGS="-- --nocapture --test-threads=1"
```

Ảnh phải thấy 9 dòng `test … ok` và dòng `test result: ok. 9 passed; 0 failed`. Chụp một ảnh gồm các dòng quan trọng (thu nhỏ cỡ chữ hoặc chụp hai lần rồi ghép nếu quá dài):

| Test | Dòng log kỳ vọng |
|---|---|
| `test_unlock_with_correct_preimage` | `consume cycles: 33572` |
| `test_wrong_preimage_is_rejected` | `HashMismatch (11)` |
| `test_empty_witness_lock_field_is_rejected` | `MissingPreimage (10)` |
| `test_missing_witness_is_rejected` | `IndexOutOfBound (1)` |
| `test_args_must_be_exactly_32_bytes` | `InvalidArgsLength (12)` |
| `test_output_lock_is_never_executed` | `ok` (không có log lỗi) |
| `test_inputs_with_same_lock_run_as_one_group` | `cycles: 1 input = 33572, 2 inputs in one group = 33572` |
| `test_inputs_with_different_locks_run_as_separate_groups` | `HashMismatch (11)` (nhánh cố ý sai) rồi `ok` |
| `tests::test_hello_world` | `consume cycles: 7182` |

### A7. Phát lại giao dịch lỗi → `rust-lock-debugger.png`

```bash
rm -rf tests/failed_txs
make test CARGO_ARGS="-- test_wrong_preimage_is_rejected --nocapture"
ls tests/failed_txs
ckb-debugger -f "$(ls -t tests/failed_txs/*.json | head -1)" -i 0 -t input -s lock
```

Ảnh phải thấy: dòng `Failed tx written to …/tests/failed_txs/0x….json`, rồi kết quả debugger `Script log: simple-lock rejected the transaction: HashMismatch (11)`, `Run result: 11`, `All cycles: 35957(35.1K)`. (`echo $?` in `254` là bình thường; đó là cách debugger trả mã.)

Dọn dẹp: `rm -rf tests/failed_txs` (thư mục này đã nằm trong `.gitignore`), và `rm -rf ~/ckb-demo` khi xong A4.

---

## Phần B — Tutorial #5: Build a Simple Lock (ảnh 8–16)

Thư mục làm việc (T3): `cd ~/Documents/CKBuilder/week-5/src/simple-lock`.

### B1. Build contract → `js-build.png`

```bash
pnpm install
pnpm build
ls -l dist
```

Ảnh phải thấy: `Contract 'hash-lock' built successfully!` và `dist/hash-lock.bc` (kỳ vọng 27810 byte).

Nếu `pnpm install` báo `ERR_PNPM_IGNORED_BUILDS`: file `pnpm-workspace.yaml` đã có sẵn mục `allowBuilds` (esbuild và unrs-resolver `true`, secp256k1 `false`); chạy lại `pnpm install` là đủ.

### B2. Chạy devnet và deploy → `js-deploy.png`

T2:

```bash
offckb node
```

Chờ dòng `CKB devnet is ready at http://127.0.0.1:8114.` (để chạy suốt Phần B). Devnet này giữ trạng thái từ Tuần 1; không chạy `offckb clean` trừ khi bạn thật sự muốn chain mới.

T3:

```bash
pnpm run deploy -- --network devnet
```

Lệnh hỏi `Are you sure you want to deploy these contracts? (Y/n)`: gõ `Y` + Enter (phải chạy trong terminal thật, không qua pipe). Ảnh phải thấy: `tx committed.`, `Deployment completed successfully!`, `Contract OutPoint: 0x…:0`, `ckb-js-vm OutPoint: 0x…:15`. **Ghi lại hai OutPoint vào báo cáo** (dòng #8 trong báo cáo).

`git status` sau đó sẽ hiện `deployment/scripts.json`, `frontend/deployment/*.json` đổi và một file mới trong `deployment/devnet/hash-lock.bc/migrations/`; đây là hồ sơ deploy của bạn, giữ lại để commit.

### B3. Frontend → `js-ui-ready.png`

T4:

```bash
cd ~/Documents/CKBuilder/week-5/src/simple-lock/frontend
pnpm dev
```

Mở http://localhost:3000 (nếu đã mở trước khi deploy, tải lại trang). Thu nhỏ trang (`Ctrl` + `-`) để thấy được nhiều mục. Giữ preimage mặc định `Hello World`. Ảnh phải thấy:

- Deployment health: `DEVNET` / `READY`, Contract code hash `0xcd262cb3…d23519`, hai OutPoint trùng với B2.
- blake2b-256: `0x106911e4f83e790e1eb2f39bdff23c1db43ed5af9219f763e571389af21259ca`.
- Hash-lock address (`ckt1q…`) và khối Lock script (`hashType` `type`, `args` bắt đầu bằng `0x0000cd262cb3…` rồi `04…`, rồi `106911e4…`).

Chép địa chỉ hash-lock vào báo cáo (dòng #9). Nếu trạng thái không phải READY: bấm "Check again"; nếu vẫn lỗi, deploy lại (B2) và khởi động lại `pnpm dev`.

### B4. Nạp tiền → `js-deposit.png` và `js-ui-funded.png`

T3 (thay `<ADDRESS>` bằng địa chỉ hash-lock đã chép):

```bash
offckb deposit --network devnet <ADDRESS> 300
```

Ảnh `js-deposit.png`: lệnh và dòng `tx hash:  0x…`. **Ghi tx hash vào báo cáo** (dòng #10).

Chờ giao dịch committed (thường 10–20 giây):

```bash
TX=0x...   # tx hash vừa nhận
curl -s -X POST -H 'content-type: application/json' \
  -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"get_transaction\",\"params\":[\"$TX\"]}" \
  http://127.0.0.1:8114 | jq -r '.result.tx_status.status'
```

Khi in `committed`, bấm **Refresh committed balance** trong UI. Ảnh `js-ui-funded.png`: "Total live capacity" tăng thêm 300 CKB, và vẫn thấy địa chỉ.

> **Số dư thực tế của bạn là 500,99999506 CKB, không phải 300.** Cùng preimage `Hello World` luôn cho cùng một địa chỉ, và devnet giữ trạng thái giữa các lần chạy, nên địa chỉ này còn 200,99999506 CKB tiền thối từ lần chạy thử của Claude (cũng là lý do giao dịch unlock ở B7 có 2 input). Báo cáo đã ghi đúng theo ảnh của bạn.

### B5. Preimage sai → `js-ui-wrong.png`

Ở mục "Preimage to reveal" nhập `wrong secret`, bấm **Transfer CKB**. Ảnh phải thấy: `Transaction status: failed` và dòng "The preimage does not match this hash lock (error 11). The transaction was rejected atomically and the original cells remain live." Sau đó bấm Refresh để chắc chắn số dư không đổi (xem Phần E về số dư hiện tại).

### B6. Preimage đúng → `js-ui-unlock.png`

Nhập lại `Hello World` ở "Preimage to reveal", giữ Amount mặc định 99, bấm **Transfer CKB**. Trạng thái chuyển `pending` rồi `committed` (cần vài giây). Ảnh phải thấy `Transaction status: committed` và `Transaction hash: 0x…`. **Ghi tx hash vào báo cáo** (dòng #11).

### B7. Xem giao dịch unlock bằng RPC → `js-rpc-unlock-tx.png`

```bash
TX=0x...   # tx hash ở B6
curl -s -X POST -H 'content-type: application/json' \
  -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"get_transaction\",\"params\":[\"$TX\"]}" \
  http://127.0.0.1:8114 \
  | jq '{status:.result.tx_status.status, block:.result.tx_status.block_number, cycles:.result.cycles, inputs:.result.transaction.inputs, outputs:(.result.transaction.outputs|map({capacity,lock_code_hash:.lock.code_hash})), cell_deps:.result.transaction.cell_deps, witnesses:.result.transaction.witnesses}'
python3 -c "print(bytes.fromhex('48656c6c6f20576f726c64').decode())"
```

Kỳ vọng (từ lần chạy thử; của bạn có thể lệch chút):

- `status` `committed`; `cycles` `0xc7a5d4` (= 13 084 116).
- `inputs`: một input, `tx_hash` chính là tx hash nạp tiền ở B4.
- `outputs`: #0 `0x24e160300` (99 CKB) lock code hash `0x9bd7e06f…` (lock mặc định); #1 `0x4ae0da712` (200,99999506 CKB) lock code hash `0xac4db0d0…` (ckb-js-vm, tức là tiền thối quay về hash lock).
- `cell_deps`: hai mục (bytecode `hash-lock.bc` và ckb-js-vm, index `0xf`).
- `witnesses[0]` kết thúc bằng `…48656c6c6f20576f726c64`; lệnh `python3` in ra `Hello World`: preimage nằm công khai trong witness.

Tính phí để điền báo cáo: `300 − 99 − 200,99999506 = 0,00000494 CKB` (494 shannon). Nếu số của bạn khác, sửa dòng #12.

### B8. Test của tutorial → `js-tests.png`

```bash
pnpm test
```

Ảnh phải thấy: `PASS tests/frontend-transaction.test.ts`, `PASS tests/hash-lock.mock.test.ts`, khối log `Run result: 0` / `All cycles: 13086867(12.5M)`, và `Tests: 12 passed`.

Xong Phần B: `Ctrl+C` ở T4 (frontend) và T2 (`offckb node`). Kiểm tra không còn tiến trình `ckb` mồ côi: `pgrep -af "bins/.*/ckb"`; nếu còn, `kill` đúng PID đó.

---

## Phần C — Academy A4, A5 (ảnh `academy-a4.png`, `academy-a5.png`)

Phần này Claude không làm hộ được (cần tài khoản Academy của bạn), và cũng chưa xem được giao diện Academy (trang web cần JavaScript và đăng nhập), nên các bước dưới đây là hướng dẫn chung:

1. Vào https://academy.ckb.dev/courses, đăng nhập.
2. **A4 — L1 Developer Training Course.** Theo giáo trình tuần 5, học các bài về Cell Model và giao dịch. Nội dung tương ứng trên bản [gitbook](https://nervos.gitbook.io/developer-training-course/lab-exercise-setup): *Transactions* (Transaction Lifecycle…) và *Scripting Basics* (Introduction to Scripting Part 1–2, Validating a Transaction, Syscalls and Sources, Script Groups, Using Script Args). Bỏ qua phần lab dùng Lumos/Capsule (Handbook cho phép).
3. **A5 — CKB Script Programming Series.** Nội dung trùng Class 1–2 của Script course (Intro to Script, Program Languages for Script).
4. Chụp trang khóa học cho thấy: tên khóa học, các bài đã hoàn thành hoặc phần trăm tiến độ, và điểm nếu Academy hiển thị. Lưu thành `academy-a4.png` và `academy-a5.png`.
5. Báo cáo hiện không có mục Academy (cả phần Completed lẫn Evidence); nếu muốn đưa vào, thêm một mục `- Academy:` vào Completed và hai dòng mới ở cuối bảng Evidence. Nếu bạn mới học một phần, ghi đúng phần đã xong (A4 và A5 kéo dài tuần 5–7).

---

## Phần D — Hoàn thiện báo cáo

### D1. Kiểm tra ảnh và TODO

```bash
cd ~/Documents/CKBuilder/week-5
grep -o 'imgs/[a-z0-9-]*\.png' report.md | sort -u | while read f; do
  [ -f "$f" ] && echo "OK       $f" || echo "MISSING  $f"
done
grep -n "TODO" report.md
```

Không được còn `MISSING`. Mọi `TODO(...)` phải được thay bằng giá trị của bạn (bảng bên dưới); xóa luôn hai khối chú thích `<!-- … -->` khi xong.

| Vị trí | Giá trị cần điền | Lấy ở |
|---|---|---|
| Evidence #8 | `Contract OutPoint`, `ckb-js-vm OutPoint` | B2 |
| Evidence #9 | Địa chỉ hash-lock `ckt1…` | B3 |
| Evidence #10 | Tx hash của lần nạp tiền | B4 |
| Evidence #11 | Tx hash của lần unlock | B6 |

### D2. Đối chiếu và viết ý kiến riêng

- Mở từng ảnh cạnh dòng tương ứng trong bảng Evidence; số nào lệch thì sửa theo ảnh (mục "Về nguồn gốc các con số" ở đầu file).
- Đọc phần Key learnings, sửa theo cách diễn đạt của bạn; thêm cảm nhận riêng ở cuối (chỗ có comment `TODO(user)`), ví dụ: điều gì bất ngờ (ví dụ chênh lệch cycles giữa JS và Rust, lock chỉ chạy khi spend), điều còn chưa rõ, câu hỏi cho Neon.
- Câu tự kiểm tra của giáo trình: *Lock Script chạy khi cell được tạo hay khi bị tiêu? Type Script chạy khi nào?* Trả lời nằm ở learning #3; hãy thử tự diễn đạt lại mà không nhìn báo cáo.

### D3. Commit

Chỉ làm khi bạn đã hài lòng. Repo dùng nhánh `week-5`; các tuần trước commit với thông điệp dạng `Week N`.

```bash
cd ~/Documents/CKBuilder
git status --short week-5 | head -50
git add week-5
git status --short | grep -E "\.privkey|\.env$" && echo "CHÚ Ý: có file bí mật" || echo "không có file bí mật"
```

`target/`, `build/`, `node_modules/`, `dist/`, `tests/failed_txs/` đã được `.gitignore` bỏ qua. Hai file `docs/CKB Builder Handbook.md` và `docs/CKBuilder T&Cs.md` đang có thay đổi chưa commit từ trước (không thuộc bước này); đừng `git add` chúng nếu không chủ ý.

---

## Phần E — Chụp bổ sung (sau khi xem 13 ảnh đầu)

13 ảnh đầu đã được đối chiếu và điền vào `report.md`; các dòng Evidence của những ảnh chưa chụp đã bị xóa khỏi báo cáo. Còn lại:

| Ảnh | Tình trạng | Cách chụp |
|---|---|---|
| `rust-generate-workspace.png` | Chưa có | Làm A2 (thư mục tạm `~/ckb-demo`, mất ~1 phút). Dòng Evidence tương ứng đã bị xóa khỏi báo cáo; nếu chụp, thêm lại một dòng mới. |
| `rust-lock-debugger.png` | Có ảnh nhưng **thiếu kết quả của `ckb-debugger`**: ảnh bị cắt ở phần log `make test` | Chụp lại với lệnh lọc cho gọn (xem bên dưới). |
| `js-ui-wrong.png` | Chưa có | Xem bên dưới. |
| `js-tests.png` | Chưa có | Làm B8 (`pnpm test`; không cần devnet). |
| `academy-a4.png`, `academy-a5.png` | Chưa có | Làm Phần C. |

**Chụp lại `rust-lock-debugger.png`** (T1 trong `week-5/src/ckb-rust-script`, đã `export` biến ở mục 0.3):

```bash
rm -rf tests/failed_txs
make test CARGO_ARGS="-- test_wrong_preimage_is_rejected --nocapture" 2>&1 | grep -E "contract debug|Failed tx|^test "
ckb-debugger -f "$(ls -t tests/failed_txs/*.json | head -1)" -i 0 -t input -s lock
```

Ảnh phải thấy: dòng `Failed tx written to …`, rồi `Script log: simple-lock rejected the transaction: HashMismatch (11)`, `Run result: 11`, `All cycles: 35957(35.1K)`. Sau khi chụp, cập nhật dòng #6 trong báo cáo: thêm kết quả `Run result: 11`, `All cycles: 35957(35.1K)` và đổi `0xa59c0f36….json` theo tên file trong ảnh mới.

**Chụp `js-ui-wrong.png`** (cần chạy lại devnet và frontend):

```bash
# T2
offckb node
# T4
cd ~/Documents/CKBuilder/week-5/src/simple-lock/frontend && pnpm dev
```

Mở http://localhost:3000 (preimage vẫn `Hello World`, cùng địa chỉ cũ). "Total live capacity" lúc này là **401,99998968 CKB** (tiền thối của giao dịch unlock ở B6). Nhập `wrong secret` vào "Preimage to reveal", bấm **Transfer CKB**. Ảnh phải thấy `Transaction status: failed` và dòng "The preimage does not match this hash lock (error 11)…". Bấm Refresh để thấy số dư vẫn 401,99998968 CKB rồi cập nhật dòng #12 (đã ghi "Capacity stays unchanged"; thêm con số này nếu muốn). Xong thì `Ctrl+C` hai terminal, và kiểm tra `pgrep -af "bins/.*/ckb"`.

**Không bắt buộc:** `js-ui-ready.png` hiện có dòng nổi "To exit full screen, press and hold Esc" của trình duyệt che tiêu đề. Nếu muốn ảnh sạch, thoát toàn màn hình rồi chụp lại; số dư lúc đó là 401,99998968 CKB nên báo cáo (dòng #9) sẽ cần sửa theo.

---

## Xử lý lỗi thường gặp (đều đã gặp khi chạy thử)

| Triệu chứng | Nguyên nhân | Cách xử lý |
|---|---|---|
| `Cannot find clang of version 16+!` hoặc `Clang compiler not found` khi `make build` | Clang 14 < 16, hoặc chưa đặt `CLANG` | Mục 0.3 |
| `failed to find tool "llvm-ar"` | `llvm-ar` không có trên `PATH` | `export PATH=/usr/lib/llvm-14/bin:$PATH` |
| `ckb-debugger: command not found` | Chưa cài | `offckb install ckb-debugger` |
| `cargo install ckb-debugger` lỗi `Could not find protoc` | Thiếu `protobuf-compiler` | Dùng `offckb install ckb-debugger`, hoặc `sudo apt install protobuf-compiler` rồi cài lại |
| `Binary "…/build/release/simple-lock" is missing!` khi `make test` | Chưa build | `make build` trước |
| Test `tests::test_simple_lock` fail với `error code 12` | Test mẫu do `make generate CRATE=simple-lock` sinh ra dùng args 1 byte | Xóa test mẫu (repo đã xóa sẵn) |
| `ERR_PNPM_IGNORED_BUILDS` | pnpm 12 chặn build script | Đã cấu hình trong `pnpm-workspace.yaml`; chạy lại `pnpm install` |
| Deploy báo `User force closed the prompt` | Chạy không có terminal tương tác | Chạy trong terminal thật, gõ `Y` |
| UI báo dependency stale/not ready | OutPoint cũ (deploy lại, hoặc devnet bị reset) | Deploy lại (B2), khởi động lại `pnpm dev` |
| Nút Transfer CKB bị khóa | Chưa có số dư hash lock | Chờ deposit committed rồi Refresh |
| `offckb node` báo cổng 8114 đang dùng | Còn node cũ chạy | `pgrep -af "bins/.*/ckb"`, kill đúng PID |

## Không bắt buộc (stretch)

Tutorial #5 có nhánh testnet (`pnpm run deploy -- --network testnet --privkey 0x…`, `pnpm dev:testnet`). Lưu ý: `hash-lock.bc` nặng 27 810 byte nên cell chứa code khóa khoảng 27,9 nghìn CKB testnet, và private key nằm trong lịch sử shell; vì giáo trình tuần 5 chỉ yêu cầu devnet và tuần 7 sẽ deploy lên testnet đúng chuẩn (Type ID), nên để dành.
