# CCC wallet demo (testnet)

A small Next.js (App Router) dApp that uses [`@ckb-ccc/connector-react`](https://docs.ckbccc.com/en/docs/packages/core-packages/connector-react) to connect a wallet, show its balance and transfer CKB on **testnet**.

```bash
npm install
npm run dev   # http://localhost:3000
npm run lint  # tsc --noEmit
npm run build
```

- No private key is used. The wallet signs, so the page never sees the key.
- The wallet must be switched to testnet and hold test CKB from the [faucet](https://faucet.nervos.org/).
- The whole logic is in [app/page.tsx](app/page.tsx): `ccc.Provider` → `ccc.useCcc` / `ccc.useSigner` → `Transaction.from` → `completeInputsByCapacity` → `completeFeeBy` → `sendTransaction`.
