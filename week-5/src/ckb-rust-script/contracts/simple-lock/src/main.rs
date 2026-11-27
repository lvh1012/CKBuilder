#![cfg_attr(not(any(feature = "library", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "library", test))]
extern crate alloc;

#[cfg(not(any(feature = "library", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "library", test)))]
// 16KB fixed heap + 1.2MB dynamic heap, 64-byte minimum block (ckb-std default).
ckb_std::default_alloc!(16384, 1258306, 64);

use ckb_hash::blake2b_256;
use ckb_std::{ckb_constants::Source, error::SysError, high_level};

/// Hash lock: a cell can be spent by whoever reveals the preimage of the hash
/// stored in the Script `args`.
///
/// * `args`                  = blake2b-256(preimage), exactly 32 bytes
/// * `witness.lock` (WitnessArgs) = the preimage
///
/// Educational only: the preimage is public once it is used, so this proves
/// knowledge of a secret, not ownership.
///
/// The exit code is the only thing a Script returns: 0 = valid, anything else
/// rejects the whole transaction. The codes below are the contract's public API.
#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    // Mirror of the syscall errors (1..=4); see "Common Error Codes" in the docs.
    IndexOutOfBound = 1,
    ItemMissing = 2,
    LengthNotEnough = 3,
    Encoding = 4,
    // Any other syscall error.
    Unknown = 9,
    // Custom errors.
    /// `WitnessArgs.lock` is empty, so there is no preimage to check.
    MissingPreimage = 10,
    /// blake2b-256(preimage) != args.
    HashMismatch = 11,
    /// `args` is not exactly 32 bytes.
    InvalidArgsLength = 12,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        match err {
            SysError::IndexOutOfBound => Self::IndexOutOfBound,
            SysError::ItemMissing => Self::ItemMissing,
            SysError::LengthNotEnough(_) => Self::LengthNotEnough,
            SysError::Encoding => Self::Encoding,
            _ => Self::Unknown,
        }
    }
}

pub fn program_entry() -> i8 {
    match check_hash() {
        Ok(()) => 0,
        Err(err) => {
            ckb_std::debug!(
                "simple-lock rejected the transaction: {:?} ({})",
                err,
                err as i8
            );
            err as i8
        }
    }
}

fn check_hash() -> Result<(), Error> {
    // 1. The expected hash comes from the Script of the cell being unlocked.
    let script = high_level::load_script()?;
    let args = script.args().raw_data();
    let expected: [u8; 32] = args
        .as_ref()
        .try_into()
        .map_err(|_| Error::InvalidArgsLength)?;

    // 2. The preimage comes from the witness at the same position as the first
    //    input of this Script group (GroupInput, not Input: other inputs in the
    //    transaction may belong to other Scripts).
    let witness_args = high_level::load_witness_args(0, Source::GroupInput)?;
    let preimage = witness_args
        .lock()
        .to_opt()
        .ok_or(Error::MissingPreimage)?
        .raw_data();

    // 3. Compare.
    if blake2b_256(preimage.as_ref()) == expected {
        Ok(())
    } else {
        Err(Error::HashMismatch)
    }
}
