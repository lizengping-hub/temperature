use crate::ReceiveOwner;
use anchor_lang::prelude::*;
use vipers::validate::Validate;
use vipers::assert_keys_eq;
impl<'info> Validate<'info> for ReceiveOwner<'info> {
    fn validate(&self) -> ProgramResult {
        assert_keys_eq!(
            self.temperature.pending_owner,
            self.pending_owner.key(),
            "pool.pending_owner"
        );
        Ok(())
    }
}

impl ReceiveOwner<'_> {
    pub fn process(
        ctx: Context<ReceiveOwner>,
    ) -> ProgramResult {
        let clock = Clock::get()?;
        clock.unix_timestamp;
        ctx.accounts.temperature.owner = ctx.accounts.temperature.pending_owner;
        Ok(())
    }


}

