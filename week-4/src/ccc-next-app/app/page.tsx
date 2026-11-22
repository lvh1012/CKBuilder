"use client";

import { useCallback, useEffect, useState } from "react";
import { ccc } from "@ckb-ccc/connector-react";

const EXPLORER_TX = "https://pudge.explorer.nervos.org/transaction/";

// A cell with the default secp256k1 lock needs 8 (capacity) + 53 (lock) bytes = 61 CKB,
// even with no data, so the node rejects any smaller payment output.
const MIN_CELL_CKB = 61;

/**
 * Wallet panel: connect, show address and balance, send CKB.
 * Must render inside <ccc.Provider>, because useCcc/useSigner read its context.
 */
function Wallet() {
  const { open, disconnect, wallet } = ccc.useCcc();
  const signer = ccc.useSigner();

  const [address, setAddress] = useState("");
  const [balance, setBalance] = useState("");
  const [to, setTo] = useState("");
  const [amount, setAmount] = useState("62");
  const [txHash, setTxHash] = useState("");
  const [status, setStatus] = useState("");

  const refresh = useCallback(async () => {
    if (!signer) return;
    setAddress(await signer.getRecommendedAddress());
    // getBalance returns shannons (1 CKB = 1e8 shannons); fixedPointToString converts to CKB.
    setBalance(ccc.fixedPointToString(await signer.getBalance()));
  }, [signer]);

  useEffect(() => {
    refresh().catch((e) => setStatus(`Cannot load wallet: ${e.message ?? e}`));
  }, [refresh]);

  async function send() {
    if (!signer) return;
    setTxHash("");
    try {
      // Both parse calls throw on bad input, which is the validation at this trust boundary.
      const { script: lock } = await ccc.Address.fromString(to, signer.client);
      const capacity = ccc.fixedPointFrom(amount);
      if (capacity < ccc.fixedPointFrom(MIN_CELL_CKB)) {
        throw new Error(`Amount must be at least ${MIN_CELL_CKB} CKB`);
      }

      // The order matters: outputs, then inputs (so the fee is computed on the final size), then fee.
      const tx = ccc.Transaction.from({ outputs: [{ lock, capacity }] });
      await tx.completeInputsByCapacity(signer);
      await tx.completeFeeBy(signer);

      setStatus("Waiting for wallet approval...");
      const hash = await signer.sendTransaction(tx);
      setTxHash(hash);
      setStatus("Sent. Waiting for 1 confirmation...");
      await signer.client.waitTransaction(hash, 1);
      setStatus("Committed.");
      await refresh();
    } catch (e) {
      setStatus(`Failed: ${e instanceof Error ? e.message : String(e)}`);
    }
  }

  if (!signer) {
    return <button onClick={open}>Connect wallet</button>;
  }

  return (
    <div style={{ display: "grid", gap: 12, maxWidth: 640 }}>
      <div>
        Wallet: <b>{wallet?.name}</b>{" "}
        <button onClick={disconnect}>Disconnect</button>
      </div>
      <div style={{ wordBreak: "break-all" }}>Address: {address}</div>
      <div>Balance: {balance || "..."} CKB</div>

      <input
        placeholder="Recipient address (ckt1...)"
        value={to}
        onChange={(e) => setTo(e.target.value)}
      />
      <input
        placeholder="Amount in CKB"
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
      />
      <button onClick={send} disabled={!to}>
        Transfer
      </button>

      {status && <div>{status}</div>}
      {txHash && (
        <a href={EXPLORER_TX + txHash} target="_blank" rel="noreferrer">
          {txHash}
        </a>
      )}
    </div>
  );
}

export default function Home() {
  return (
    // The Provider defaults to testnet; the wallet must be switched to testnet too.
    <ccc.Provider name="CKBuilder Week 4">
      <main style={{ padding: 24 }}>
        <h1>CCC wallet demo (testnet)</h1>
        <Wallet />
      </main>
    </ccc.Provider>
  );
}
