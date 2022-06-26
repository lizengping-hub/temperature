//! Math for preserving precision of token amounts which are limited
//! by the SPL Token program to be at most u64::MAX.
//!
//! Decimals are internally scaled by a WAD (10^18) to preserve
//! precision up to 18 decimal places. Decimals are sized to support
//! both serialization and precise math for the full range of
//! unsigned 64-bit integers. The underlying representation is a
//! u192 rather than u256 to reduce compute cost while losing
//! support for arithmetic operations at the high end of u64 range.

#![allow(clippy::assign_op_pattern)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(clippy::manual_range_contains)]

use crate::{
    error::ErrorCode,
    math::{common::*},
};
pub use borsh::BorshDeserialize;
pub use borsh::BorshSerialize;
use std::{fmt};
use anchor_lang::solana_program::program_error::ProgramError;

use uint::construct_uint;

// U192 with 192 bits consisting of 3 x 64-bit words

construct_uint! {
    #[derive(BorshDeserialize,BorshSerialize)]
    pub struct U192(3);
}


/// Large decimal values, precise to 18 digits
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord, BorshDeserialize,BorshSerialize)]
pub struct Decimal(pub U192);

impl Decimal {

    // OPTIMIZE: use const slice when fixed in BPF toolchain
    fn wad() -> U192 {
        U192::from(WAD)
    }

}
impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut scaled_val = self.0.to_string();
        if scaled_val.len() <= SCALE {
            scaled_val.insert_str(0, &vec!["0"; SCALE - scaled_val.len()].join(""));
            scaled_val.insert_str(0, "0.");
        } else {
            scaled_val.insert(scaled_val.len() - SCALE, '.');
        }
        f.write_str(&scaled_val)
    }
}
impl From<u64> for Decimal {
    fn from(val: u64) -> Self {
        Self(Self::wad() * U192::from(val))
    }
}

impl TryDiv<u64> for Decimal {
    fn try_div(self, rhs: u64) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_div(U192::from(rhs))
                .ok_or(ErrorCode::MathOverflow)?,
        ))
    }
}
impl TryDiv<Decimal> for Decimal {
    fn try_div(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_mul(Self::wad())
                .ok_or(ErrorCode::MathOverflow)?
                .checked_div(rhs.0)
                .ok_or(ErrorCode::MathOverflow)?,
        ))
    }
}
impl TryMul<u64> for Decimal {
    fn try_mul(self, rhs: u64) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_mul(U192::from(rhs))
                .ok_or(ErrorCode::MathOverflow)?,
        ))
    }
}
impl TryAdd for Decimal {
    fn try_add(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_add(rhs.0)
                .ok_or(ErrorCode::MathOverflow)?,
        ))
    }
}
impl TrySub for Decimal {
    fn try_sub(self, rhs: Self) -> Result<Self, ProgramError> {
        Ok(Self(
            self.0
                .checked_sub(rhs.0)
                .ok_or(ErrorCode::MathOverflow)?,
        ))
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scaler() {
        assert_eq!(U192::exp10(SCALE), Decimal::wad());
    }
    #[test]
    fn test_one() {
        assert_eq!(Decimal::one(),Decimal(Decimal::wad()))
    }
}
