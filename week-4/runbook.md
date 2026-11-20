# Week 4 Runbook: CCC và TypeScript dApps

Bạn chạy từng bước, chụp screenshot theo bảng ở cuối, rồi điền `TODO(user)` trong [report.md](report.md).
Thời gian: Core ~5h (mục 1 đến 4), Stretch ~2–3h (mục 5).

> **Đã kiểm chứng bởi tôi (chỉ trên devnet, không dùng key của bạn):** `npm install`, `npm run lint` và logic của Tutorial #2, #3 chạy đúng; app Next.js `lint` + `build` pass, trang render và modal connect mở được. **Chưa kiểm chứng:** mọi thứ trên testnet, faucet, kết nối ví thật, và Playground với ví thật.

## 0. Chuẩn bị

**Quy tắc an toàn:** chỉ dùng key testnet dùng một lần. Không chụp private key. File `.env` đã nằm trong `.gitignore`.

1. Tạo key testnet mới. Lệnh này ghi key vào `week-4/.env` và chỉ in ra địa chỉ:
   ```bash
   cd week-4/src/store-data-on-cell && npm install
   node -e "const {ccc}=require('@ckb-ccc/core');const k='0x'+require('crypto').randomBytes(32).toString('hex');require('fs').writeFileSync('../../.env','CKB_PRIVATE_KEY='+k+'\n');new ccc.SignerCkbPrivateKey(new ccc.ClientPublicTestnet(),k).getRecommendedAddress().then(a=>console.log('address:',a))"
   ```
   Mở `week-4/.env` để lấy key khi UI của tutorial hỏi (copy/paste, đừng chụp).
2. Xin CKB ở [Faucet](https://faucet.nervos.org/) cho địa chỉ vừa in. Cần khoảng 500 CKB: mỗi message cell 61 CKB trở lên, mỗi token cell 146 CKB.
3. Ví cho Playground và Next.js (mục 2 và 5): chọn một ví có sẵn trong modal (JoyID Passkey, MetaMask, OKX, UniSat, UTXO Global, Khie). Chuyển ví sang **testnet**, rồi xin CKB cho địa chỉ ví đó. Địa chỉ ví khác với địa chỉ ở bước 1 vì ví dùng lock script khác.
4. Mốc: `node -v` ≥ 20, `offckb --version` (đang là 0.4.13).

## 1. Học tài liệu (~2h)

Ba link CCC trong syllabus (`docs.ckbccc.com/docs/CCC`, `/ccc-app`, `/playground`) trả về 404. Dùng các link `/en/docs/` này:

1. [Introduction](https://docs.ckbccc.com/en/docs/getting-started/introduction), [Quick start](https://docs.ckbccc.com/en/docs/getting-started/quick-start), [Installation](https://docs.ckbccc.com/en/docs/getting-started/installation)
2. Concepts: [Cell model](https://docs.ckbccc.com/en/docs/concepts/cell-model), [Client](https://docs.ckbccc.com/en/docs/concepts/client), [Signer](https://docs.ckbccc.com/en/docs/concepts/signer), [Transaction](https://docs.ckbccc.com/en/docs/concepts/transaction), [Address](https://docs.ckbccc.com/en/docs/concepts/address)
3. Guides: [Compose Transactions](https://docs.ckbccc.com/en/docs/guides/compose-transactions), [UDT Tokens](https://docs.ckbccc.com/en/docs/guides/udt-tokens), [Connect Wallets](https://docs.ckbccc.com/en/docs/guides/connect-wallets), [CCC Playground](https://docs.ckbccc.com/en/docs/guides/playground), [Node.js Backend](https://docs.ckbccc.com/en/docs/guides/node-js-backend)
4. [Code Examples](https://docs.ckbccc.com/en/docs/code-examples), [API reference](https://api.ckbccc.com/)

Tự kiểm tra sau khi đọc:
- `Client` khác `Signer` ở đâu?
- Vì sao phải gọi `completeInputsByCapacity` trước `completeFeeBy`?
- Output không đặt `capacity` thì CCC điền bao nhiêu?

## 2. CCC Playground (~1h): hoàn thành các ví dụ

Mở https://live.ckbccc.com/. Kiểm tra thanh trên đang ở **TESTNET**. Menu **Examples** có **8 ví dụ** (tôi đã đọc từ UI):

| # | Ví dụ | Cần ví? | Ghi chú |
|---|---|---|---|
| 1 | Build a CKB transfer | Có, có CKB | Chỉ build + preview, không ký, không gửi |
| 2 | Build an xUDT transfer | Có token xUDT mẫu | Nếu ví không có token, ghi lại thông báo lỗi và dùng làm evidence |
| 3 | Query and transfer a Spore | Ví | Không có Spore thì chỉ báo kết quả rỗng |
| 4 | Query balances | Ví hoặc Example wallet | So sánh API balance của signer và client |
| 5 | Explore live cells | Ví hoặc Example wallet | Tìm cell theo lock và type, xUDT balance |
| 6 | Read transaction history | Ví hoặc Example wallet | Tối đa 10 giao dịch |
| 7 | Resolve a CKB DID | Ví hoặc Example wallet | Sửa DID mẫu cho đúng mạng nếu cần |
| 8 | Trace DID history | Ví hoặc Example wallet | DID không tồn tại thì trả lịch sử rỗng |

Cách làm mỗi ví dụ:
1. Bấm **Examples** → chọn ví dụ → xác nhận hộp thoại.
2. Chạy bằng **Step** (dừng ở mỗi `render`) cho ví dụ 1 và 2, **Run** cho các ví dụ còn lại.
3. Chụp khung **OUTPUT** (với ví dụ 1: ảnh sau mỗi `render`, thấy Inputs, Outputs, Snapshot fee thay đổi).

Quan sát của tôi (headless, chưa kết luận):
- Chạy khi chưa kết nối ví báo lỗi "This run stopped because no wallet was connected" với hai nút **Connect real wallet** / **Use example wallet** (Example wallet là READ-ONLY).
- Ví dụ 1 với Example wallet hiện snapshot đầu (Inputs 0 CKB, Outputs 100 CKB, fee "Not available") rồi **treo ở "Executing" hơn 25 giây** ở bước `completeInputsByCapacity`. Nguyên nhân chưa rõ (có thể Example wallet không có CKB testnet). Nếu bạn gặp lại, kết nối ví thật có CKB rồi chạy lại.

**Gửi thật một giao dịch (để có tx hash):** Playground mặc định không gửi. Với ví thật đã kết nối, ở ví dụ 1 thêm hai dòng cuối rồi Run:
```ts
const txHash = await signer.sendTransaction(tx);
console.log(txHash);
```
Duyệt giao dịch trong ví, rồi mở tx hash trên [Testnet Explorer](https://pudge.explorer.nervos.org/). Đây là giao dịch bắt buộc phải có để chứng minh bạn đã làm xong Playground.

## 3. Tutorial #2: Store Data on Cell (~1h)

Code ở [src/store-data-on-cell](src/store-data-on-cell/) (copy từ `nervosnetwork/docs.nervos.org`, `examples/dApp/store-data-on-cell`). Logic nằm trong [lib.ts](src/store-data-on-cell/lib.ts): `buildMessageTx` và `readOnChainMessage`.

1. Dry run trên devnet (tùy chọn): terminal 1 `offckb node`; terminal 2 `cd week-4/src/store-data-on-cell && npm install && npm start`; mở http://localhost:1234; dán key từ `offckb accounts --show-private-keys` (tài khoản #0).
2. Testnet: dừng app, chạy `npm run start:testnet`, dán key trong `.env`.
3. Lưu message `hi` → ghi tx hash → bấm Read, kiểm tra message đọc lại đúng → mở tx trên Explorer.
4. **Thí nghiệm capacity.** Lưu thêm message 38 byte, 100 byte và `xin chào CKB` (13 byte vì `à` là 2 byte UTF-8). Với mỗi tx, mở Explorer và ghi `Outputs[0] capacity`.

Kết quả kỳ vọng (tôi đo trên devnet, quy tắc không phụ thuộc mạng):

| Số byte data | Capacity output #0 |
|---|---|
| 2 (`hi`) | 63 CKB |
| 13 (`xin chào CKB`) | 74 CKB |
| 38 | 99 CKB |
| 100 | 161 CKB |

Công thức: capacity = 61 + số byte data, vì 8 (capacity) + 53 (lock: code_hash 32 + hash_type 1 + args 20) = 61. Output không đặt `capacity` thì CCC tự điền đúng mức tối thiểu.

Cần nhớ: `completeFeeBy(signer, 1000)` nghĩa là 1000 shannon/kB (giống tuần 3). `getCellLive(outPoint, true)` mới trả data; với `false` thì `outputData` là `0x` (tôi đã thử trên devnet).

## 4. Tutorial #3: Create Fungible Token (~1.5h)

Code ở [src/xudt](src/xudt/) (`examples/dApp/xudt`). Logic ở [lib.ts](src/xudt/lib.ts): `issueToken`, `queryIssuedTokenCells`, `transferTokenToAddress`.

1. Dry run devnet (tùy chọn): như mục 3, trong `src/xudt`.
2. Testnet: `cd week-4/src/xudt && npm install && npm run start:testnet`, mở http://localhost:1234.
3. **Issue** 1000 token bằng key trong `.env` → ghi tx hash và "Token xUDT args" (truy vấn bằng đúng giá trị này, không dùng `args` trong JSON của lock). Chờ giao dịch committed rồi bấm Query.
4. **Transfer** 300 token sang một địa chỉ testnet khác (ví dụ địa chỉ ví đã kết nối ở mục 5) → ghi tx hash.
5. Mở cả hai tx trên Explorer, kiểm tra từng output: lock, type script (xUDT), data.

Kết quả kỳ vọng (tôi đo trên devnet):

| Giao dịch | Output | Data (uint128 LE) | Capacity |
|---|---|---|---|
| Issue 1000 | #0 token cho issuer | 1000 | 146 CKB |
| Transfer 300 | #0 token cho người nhận | 300 | 146 CKB |
| Transfer 300 | #1 token change về issuer | 700 | 146 CKB |

- `type.args` dài 36 byte = 32 byte lock hash của issuer + 4 byte flags `00000000`.
- 146 = 8 + 53 (lock) + 69 (type: 32 + 1 + 36) + 16 (data).
- Số dư không nằm ở bảng nào: `queryIssuedTokenCells` cộng amount của các live cell có đúng type script (`findCellsByType`).
- `issueToken` in ra console một link tới `pudge.explorer.nervos.org` cả khi chạy devnet (chuỗi tĩnh), nên chỉ dùng link đó khi chạy testnet.

Tùy chọn: so sánh với `ccc.udt.Udt` trong [UDT Tokens guide](https://docs.ckbccc.com/en/docs/guides/udt-tokens) (`udt.transfer` → `completeBy` → `completeInputsByCapacity` → `completeFeeBy`). Lưu ý: class này nằm ở `@ckb-ccc/ccc`/`@ckb-ccc/udt`, còn tutorial dùng `@ckb-ccc/core`, nên cần cài thêm gói nếu bạn muốn thử.

## 5. Stretch: app Next.js (~2–3h)

Code ở [src/ccc-next-app](src/ccc-next-app/), toàn bộ logic trong [app/page.tsx](src/ccc-next-app/app/page.tsx).

```bash
cd week-4/src/ccc-next-app
npm install
npm run dev     # http://localhost:3000
```

1. Bấm **Connect wallet**, chọn ví đã chuẩn bị ở mục 0.3 (modal có Khie, JoyID Passkey, MetaMask, OKX Wallet, UniSat, UTXO Global Wallet; tôi chưa kết nối thử).
2. Kiểm tra địa chỉ và balance hiện ra (balance đọc bằng `signer.getBalance()`, đổi từ shannon sang CKB).
3. Chuyển **62** CKB tới một địa chỉ testnet khác (tối thiểu 61 CKB; app chặn nếu nhỏ hơn) → duyệt trong ví → chờ "Committed." → mở link Explorer.
4. `npm run lint` và `npm run build` phải pass (tôi đã chạy, pass).

## 6. Rust track (nếu bạn chưa biết Rust)

Rust Book chương 10 (generics, traits, lifetimes). Phần này bạn tự đọc, tôi không đưa vào báo cáo.

## 7. Bảng screenshot → dòng Evidence trong report.md

Lưu ảnh vào `week-4/imgs/` với đúng tên sau (khớp link trong report). Terminal dùng ảnh chụp cửa sổ; trình duyệt chụp cả khung có tx hash.

| Dòng | Chụp gì | Tên file |
|---|---|---|
| 1 | Terminal sau khi tạo key (chỉ địa chỉ) + trang faucet đã nhận CKB | `testnet-key-faucet.png` |
| 2–9 | Khung OUTPUT của từng ví dụ Playground 1 đến 8 | `pg-1-transfer.png`, `pg-2-xudt.png`, `pg-3-spore.png`, `pg-4-balances.png`, `pg-5-cells.png`, `pg-6-history.png`, `pg-7-did.png`, `pg-8-did-history.png` |
| 10 | Playground với `sendTransaction` + tx hash; Explorer của tx đó | `pg-send.png`, `explorer-pg-send.png` |
| 11 | Tutorial #2: UI sau khi gửi message (thấy tx hash, không thấy key) | `store-write.png` |
| 12 | Tutorial #2: UI sau Read (message đọc lại) | `store-read.png` |
| 13 | Explorer của tx message ngắn và dài (thấy capacity) | `explorer-store-short.png`, `explorer-store-long.png` |
| 14 | Tutorial #3: UI sau Issue (thấy tx hash + xUDT args) | `xudt-issue.png` |
| 15 | Explorer của tx Issue (thấy type script và data) | `explorer-xudt-issue.png` |
| 16 | Tutorial #3: UI sau Query | `xudt-query.png` |
| 17 | Tutorial #3: UI sau Transfer; Explorer của tx Transfer | `xudt-transfer.png`, `explorer-xudt-transfer.png` |
| 18 | Next.js: modal chọn ví | `next-connect.png` |
| 19 | Next.js: sau khi kết nối, thấy địa chỉ và balance | `next-balance.png` |
| 20 | Next.js: sau khi chuyển, thấy "Committed."; Explorer của tx | `next-transfer.png`, `explorer-next-transfer.png` |

Sau khi chạy xong:
1. Điền mọi `TODO(user)` trong [report.md](report.md) (`rg -n 'TODO\(user\)' week-4/report.md` để tìm hết).
2. Viết lại Key learnings bằng văn phong của bạn. Các con số trong mục 4 của Key learnings là số đo trên devnet; thay bằng số testnet của bạn nếu khác.
3. Đổi ngày tiêu đề nếu bạn nộp ngày khác 2026-11-22.
4. Xóa `week-4/.env` khi xong, và đừng commit nó (đã ignore).
