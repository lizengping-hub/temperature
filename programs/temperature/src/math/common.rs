//! Common module for Decimal and Rate


use anchor_lang::solana_program::program_error::ProgramError;
pub const SCALE: usize = 18;
/// Identity
pub const WAD: u64 = 1_000_000_000_000_000_000;

/// Try to subtract, return an error on underflow
pub trait TrySub: Sized {
    /// Subtract
    fn try_sub(self, rhs: Self) -> Result<Self, ProgramError>;
}

/// Try to subtract, return an error on overflow
pub trait TryAdd: Sized {
    /// Add
    fn try_add(self, rhs: Self) -> Result<Self, ProgramError>;
}

/// Try to divide, return an error on overflow or divide by zero
pub trait TryDiv<RHS>: Sized {
    /// Divide
    fn try_div(self, rhs: RHS) -> Result<Self, ProgramError>;
}

/// Try to multiply, return an error on overflow
pub trait TryMul<RHS>: Sized {
    /// Multiply
    fn try_mul(self, rhs: RHS) -> Result<Self, ProgramError>;
}
