use anchor_lang::prelude::*;
#[error]
pub enum ErrorCode {
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("pool error")]
    PoolError,
    #[msg("Stake Failed")]
    StakeFailed,
    #[msg("Invalid Account Input")]
    InvalidAccountInput,
    #[msg("Invalid Oracle Config")]
    InvalidOracleConfig,
    #[msg("Invoke Failed")]
    InvokeFailed,
    #[msg("Amm Not In Swap Router")]
    AmmNotInSwapRouter,
    #[msg("Amm Not Match Pool Amm And Not In Swap Router")]
    AmmNotMatchPoolAmmAndNotInSwapRouter,
    #[msg("AccountNotMatch")]
    AccountNotMatch,
    #[msg("Invalid Account Data")]
    InvalidAccountData,
    #[msg("Withdraw Failed")]
    WithdrawFailed,
    #[msg("Pool Must Be Farm")]
    PoolMustBeFarm,
    #[msg("Pool Must Not Be Farm")]
    PoolMustNotBeFarm,
    #[msg("No Enough Lp Balance")]
    NoEnoughLpBalance,
    #[msg("No Authority")]
    NoAuthority
}
