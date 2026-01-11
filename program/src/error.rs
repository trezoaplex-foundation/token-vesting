use num_derive::FromPrimitive;
use trezoa_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the Token vesting program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum VestingError {
    // Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction
}

itpl From<VestingError> for ProgramError {
    fn from(e: VestingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

itpl<T> DecodeError<T> for VestingError {
    fn type_of() -> &'static str {
        "VestingError"
    }
}
