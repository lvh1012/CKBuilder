//! Tests for the `simple-lock` hash lock.
//!
//! Each test builds a whole transaction in memory (`ckb-testtool`) and runs the
//! real RISC-V binary from `build/release/simple-lock` in CKB-VM. Besides the
//! happy and attack paths, three tests pin down how CKB runs Scripts:
//!   * a Lock Script runs on *inputs* only (`test_output_lock_is_never_executed`),
//!   * inputs with the same Script form one group that runs once
//!     (`test_inputs_with_same_lock_run_as_one_group`),
//!   * different Scripts form different groups
//!     (`test_inputs_with_different_locks_run_as_separate_groups`).

use super::*;
use ckb_testtool::{
    ckb_hash::blake2b_256,
    ckb_types::{
        bytes::Bytes,
        core::{ScriptHashType, TransactionBuilder, TransactionView},
        packed::{Bytes as PackedBytes, *},
        prelude::*,
    },
    context::Context,
};

const MAX_CYCLES: u64 = 10_000_000;
const PREIMAGE: &[u8] = b"this is my secret preimage";
const OTHER_PREIMAGE: &[u8] = b"another secret";

// Exit codes of contracts/simple-lock (enum `Error`).
const ERR_INDEX_OUT_OF_BOUND: i8 = 1;
const ERR_MISSING_PREIMAGE: i8 = 10;
const ERR_HASH_MISMATCH: i8 = 11;
const ERR_INVALID_ARGS_LENGTH: i8 = 12;

/// Deploys the contract and returns a function-like helper bundle.
struct Env {
    context: Context,
    out_point: OutPoint,
}

impl Env {
    fn new() -> Self {
        let mut context = Context::default();
        let out_point = context.deploy_cell(Loader::default().load_binary("simple-lock"));
        Env { context, out_point }
    }

    /// A hash lock whose `args` are blake2b-256(preimage).
    fn lock_for(&mut self, preimage: &[u8]) -> Script {
        self.lock_with_args(Bytes::copy_from_slice(&blake2b_256(preimage)))
    }

    fn lock_with_args(&mut self, args: Bytes) -> Script {
        self.context
            .build_script_with_hash_type(&self.out_point, ScriptHashType::Data2, args)
            .expect("script")
    }

    /// A live cell protected by `lock`, wrapped as a transaction input.
    fn input_of(&mut self, lock: &Script, capacity: u64) -> CellInput {
        let out_point = self.context.create_cell(
            CellOutput::new_builder()
                .capacity(capacity)
                .lock(lock.clone())
                .build(),
            Bytes::new(),
        );
        CellInput::new_builder().previous_output(out_point).build()
    }

    fn verify(&self, tx: &TransactionView) -> Result<u64, ckb_testtool::ckb_error::Error> {
        // Dumps the failing tx to tests/failed_txs/*.json so it can be replayed
        // with `ckb-debugger -f <file>`.
        verify_and_dump_failed_tx(&self.context, tx, MAX_CYCLES)
    }
}

fn output_with(lock: &Script, capacity: u64) -> CellOutput {
    CellOutput::new_builder()
        .capacity(capacity)
        .lock(lock.clone())
        .build()
}

/// `WitnessArgs { lock: Some(preimage) }`, serialized for `Transaction.witnesses`.
fn witness_with_preimage(preimage: &[u8]) -> PackedBytes {
    WitnessArgs::new_builder()
        .lock(Some(Bytes::copy_from_slice(preimage)).pack())
        .build()
        .as_bytes()
        .pack()
}

fn assert_script_error(err: ckb_testtool::ckb_error::Error, code: i8) {
    let message = err.to_string();
    assert!(
        message.contains(&format!("error code {code} ")),
        "expected exit code {code}, got: {message}"
    );
}

/// Single input, single output, one witness holding `preimage`.
fn spend_one_input(env: &mut Env, lock: &Script, witness: PackedBytes) -> TransactionView {
    let input = env.input_of(lock, 1000);
    let tx = TransactionBuilder::default()
        .input(input)
        .output(output_with(lock, 900))
        .output_data(Bytes::new().pack())
        .witness(witness)
        .build();
    env.context.complete_tx(tx)
}

// ---------------------------------------------------------------------------
// Happy path and attacks on the preimage
// ---------------------------------------------------------------------------

#[test]
fn test_unlock_with_correct_preimage() {
    let mut env = Env::new();
    let lock = env.lock_for(PREIMAGE);
    let tx = spend_one_input(&mut env, &lock, witness_with_preimage(PREIMAGE));

    let cycles = env.verify(&tx).expect("pass verification");
    println!("consume cycles: {cycles}");
}

#[test]
fn test_wrong_preimage_is_rejected() {
    let mut env = Env::new();
    let lock = env.lock_for(PREIMAGE);
    let tx = spend_one_input(&mut env, &lock, witness_with_preimage(OTHER_PREIMAGE));

    let err = env.verify(&tx).unwrap_err();
    assert_script_error(err, ERR_HASH_MISMATCH);
}

#[test]
fn test_empty_witness_lock_field_is_rejected() {
    let mut env = Env::new();
    let lock = env.lock_for(PREIMAGE);
    // A valid WitnessArgs whose `lock` field is absent.
    let witness = WitnessArgs::new_builder().build().as_bytes().pack();
    let tx = spend_one_input(&mut env, &lock, witness);

    let err = env.verify(&tx).unwrap_err();
    assert_script_error(err, ERR_MISSING_PREIMAGE);
}

#[test]
fn test_missing_witness_is_rejected() {
    let mut env = Env::new();
    let lock = env.lock_for(PREIMAGE);
    let input = env.input_of(&lock, 1000);
    // No witness at all: the syscall fails with IndexOutOfBound (exit code 1).
    let tx = TransactionBuilder::default()
        .input(input)
        .output(output_with(&lock, 900))
        .output_data(Bytes::new().pack())
        .build();
    let tx = env.context.complete_tx(tx);

    let err = env.verify(&tx).unwrap_err();
    assert_script_error(err, ERR_INDEX_OUT_OF_BOUND);
}

#[test]
fn test_args_must_be_exactly_32_bytes() {
    let mut env = Env::new();
    // 31 bytes: cannot be a blake2b-256 hash.
    let lock = env.lock_with_args(Bytes::from(vec![0u8; 31]));
    let tx = spend_one_input(&mut env, &lock, witness_with_preimage(PREIMAGE));

    let err = env.verify(&tx).unwrap_err();
    assert_script_error(err, ERR_INVALID_ARGS_LENGTH);
}

// ---------------------------------------------------------------------------
// How CKB executes Lock Scripts
// ---------------------------------------------------------------------------

/// A Lock Script runs when its cell is *spent* (as an input), never when a cell
/// is *created*. Here the output carries the very same lock code with `args`
/// that this Script would reject (1 byte), yet the transaction is valid.
#[test]
fn test_output_lock_is_never_executed() {
    let mut env = Env::new();
    let lock = env.lock_for(PREIMAGE);
    let broken_output_lock = env.lock_with_args(Bytes::from(vec![0xff]));

    let input = env.input_of(&lock, 1000);
    let tx = TransactionBuilder::default()
        .input(input)
        .output(output_with(&broken_output_lock, 900))
        .output_data(Bytes::new().pack())
        .witness(witness_with_preimage(PREIMAGE))
        .build();
    let tx = env.context.complete_tx(tx);

    env.verify(&tx).expect("output locks are not executed");
}

/// Inputs that share the same Script (same code, hash type and args) form ONE
/// Script group; the Script runs once for the whole group. The second input has
/// no witness at all, which would be fatal if the Script ran once per input.
#[test]
fn test_inputs_with_same_lock_run_as_one_group() {
    let mut env = Env::new();
    let lock = env.lock_for(PREIMAGE);

    // Baseline: one input.
    let single = spend_one_input(&mut env, &lock, witness_with_preimage(PREIMAGE));
    let single_cycles = env.verify(&single).expect("single input");

    // Two inputs with the same lock, one witness (only position 0 is filled).
    let input_a = env.input_of(&lock, 1000);
    let input_b = env.input_of(&lock, 1000);
    let tx = TransactionBuilder::default()
        .input(input_a)
        .input(input_b)
        .output(output_with(&lock, 1900))
        .output_data(Bytes::new().pack())
        .witness(witness_with_preimage(PREIMAGE))
        .build();
    let tx = env.context.complete_tx(tx);
    let group_cycles = env.verify(&tx).expect("one group, one execution");

    println!("cycles: 1 input = {single_cycles}, 2 inputs in one group = {group_cycles}");
    // Running the Script twice would cost roughly double; one run adds only the
    // cost of a slightly bigger transaction.
    assert!(
        group_cycles < single_cycles * 3 / 2,
        "the group should run once: {group_cycles} vs {single_cycles}"
    );
}

/// Different `args` mean different Scripts, hence different groups. Each group
/// reads `GroupInput[0]`, which maps to the witness at that input's own position
/// in the transaction.
#[test]
fn test_inputs_with_different_locks_run_as_separate_groups() {
    let mut env = Env::new();
    let lock_a = env.lock_for(PREIMAGE);
    let lock_b = env.lock_for(OTHER_PREIMAGE);

    let build = |env: &mut Env, witness_a: &[u8], witness_b: &[u8]| {
        let input_a = env.input_of(&lock_a, 1000);
        let input_b = env.input_of(&lock_b, 1000);
        let tx = TransactionBuilder::default()
            .input(input_a)
            .input(input_b)
            .output(output_with(&lock_a, 1900))
            .output_data(Bytes::new().pack())
            .witness(witness_with_preimage(witness_a))
            .witness(witness_with_preimage(witness_b))
            .build();
        env.context.complete_tx(tx)
    };

    // Each witness matches its own input: both groups succeed.
    let ok = build(&mut env, PREIMAGE, OTHER_PREIMAGE);
    env.verify(&ok).expect("both groups unlock");

    // Group B (input 1) reads the witness at index 1; give it the wrong preimage.
    let bad = build(&mut env, PREIMAGE, PREIMAGE);
    let err = env.verify(&bad).unwrap_err();
    assert_script_error(err, ERR_HASH_MISMATCH);
}
