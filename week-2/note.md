1. Cell

- The CKB blockchain works like a storage facility filled with boxes, called Cells. These Cells can hold CKBytes, the native tokens of CKB, which represents both value and storage capacity—the more CKBytes you own, the more data you can store on the blockchain
- Each Cell must reserve a minimum of 61 CKBytes
- Structure:

```
Cell: {
  capacity: HexString;  // Size of the Cell in shannon. 1 CKB = 100,000,000 shannons
  lock: Script; // A Script that enforces access and ownership of a Cell
  type: Script; // A Script that enforces the rules that must be followed in a transaction for a Cell to be consumed as an input or for a Cell to be created as an output
  data: HexString; // Used for storing states.
}
```

- Example:

```json
{
  "capacity": "0x19995d0ccf", // 8 bytes
  "lock": {
    "code_hash": "0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8", // 32 bytes
    "args": "0x0a486fb8f6fe60f76f001d6372da41be91172259", // 20 bytes
    "hash_type": "type" // 1 bytes
  },
  "type": null
}
```

- OutPoint: Identifies a specific output Cell. Its type is OutPoint, which consists of a transaction hash and an output index.

2. Script

- A Script in CKB is a binary executable that can be executed on-chain. It is a program that runs on a virtual machine powered by the RISC-V instruction set, called the CKB-VM, and can perform arbitrary logic to guard and protect your Cells. You can think of it as smart contract
- Structure:

```
Script: {
  code_hash: HexString // Hash of the binary that contains a CKB Script.
  args: HexString // Arguments as the Script input such as a user’s public key hash.
  hash_type: Uint8, there are 4 allowed values: {0: "data", 1: "type", 2: "data1", 4: "data2"} // Interpretation of the code hash when looking for matching dep cells
}
```

- There’re two main types of Scripts:

1. Lock Script: A required Script controlling the ownership and access to a Cell, ensuring only authorized users can spend/consume the Cell.
2. Type Script: An optional Script dictating how a Cell can be used or modified in a transaction. In a transaction, when input Cell and output Cell share the same Type Script, the transformation between them must follow the rules defined by that Type Script. Example: A typical use case of Type Script is in UDT. In token issuance, new tokens will be created as new Cell outputs, so a Type Script is required to validate that these new tokens are created adhere to predefined rules.

- Type Script is similar to Lock Script, with two differences:
  - Type Script is optional.
  - CKB runs Lock Script in only inputs in a transaction.
  - CKB runs Type Script in both inputs and outputs in a transaction.

- the lock code is lost: So, we know that a lock's code is usually kept in another cell. What happens if someone destroys that cell? If that cell is spent, the dep cell of a transaction becomes dead, and the lock's code will be gone => Because the cells containing the code of all CKB's built-in lock scripts are inherently inaccessible by anyone. The lock field of each built-in script code cell is

```
Never unlockable lock: {
  code_hash: 0x0000000000000000000000000000000000000000000000000000000000000000
  hash_type: data
  args: 0x
}
```

which means that no one will ever be able to unlock it, and the code will always be there

- Script Group Execution: CKB streamlines Lock Script execution by grouping the inputs based on their Lock Scripts and running the grouped Scripts only once

- CKB System Script: One of the most important system Scripts is the default Lock Script known as secp256k1_blake160_sighash_all. This Script utilizes the secp256k1 elliptic curve and the Blake2b hashing algorithm to ensure that only the owner of the corresponding private key can unlock and spend the CKBytes associated with a given Cell.

3. Transaction

- Once added to the blockchain, Cells are immutable, meaning they can’t be changed. To update a Cell’s data, you must consume it—extract the data, modify it, and create a new Cell with the updated information
- A transaction in CKB destroys some Cells (outputs from previous transactions) and generates new ones.
- Cells that have not been consumed are known as Live Cells, and these are available for use in future transactions

- Flow:

1. Create: You start by selecting a Live Cell with a capacity of 200 CKBytes. Two new Live Cells will be created as output: one Cell with 100 CKBytes, locked so only Alice can unlock it; another Cell with 99.999 CKBytes locked to yourself as your change. The 0.001 CKBytes difference will be used as the transaction fee.
2. Sign: You sign the transaction with your private key, proving that you have the authority to spend the CKBytes in this Cell.
3. Broadcast: After signing, the transaction is broadcasted from your wallet to the nearest CKB network node.
4. Validate: The receiving node performs several checks to validate your transaction
5. Propagate: The validated transaction is propagated to other nodes in the network and enters the mempool–a temporary storage area for unconfirmed transactions, waiting for miners to pick up.
6. Confirm: Miners will select your transaction from the mempool and include it in a new block. Once the block containing your transaction is mined, it’s added to the blockchain. At this point, your transaction has 1 confirmation.

- structure:
  Fields & Description
  Name Type Description
  version Uint32 Transaction version distinguishes transactions in the event of a fork.
  cell_deps [CellDep] A list of outpoint pointing to the Cells that are dependencies of this transaction. For more, see cell_deps.
  header_deps [H256(hash)] A list of H256 hashes pointing to block headers that are dependencies of this transaction. For more, see header_deps.
  inputs [CellInput] A list of referenced Cell inputs. For more, see inputs.
  witnesses [Bytes] Provided by transaction creator to ensure the successful execution of the corresponding Lock Script. For more, see witnesses.
  outputs [Cells] A list of Cells used as outputs, also can be seen as the newly generated Cells. For more, see outputs.
  outputs_data [Bytes] A list of Cell data of each output Cell. For more, see outputs_data.
