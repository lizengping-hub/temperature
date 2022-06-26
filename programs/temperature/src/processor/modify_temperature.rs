
use crate::ModifyTemperature;
use anchor_lang::prelude::*;
use vipers::validate::Validate;
use vipers::assert_keys_eq;
use crate::state::reporter::Reporter;

impl<'info> Validate<'info> for ModifyTemperature<'info> {
    fn validate(&self) -> ProgramResult {
        // This rpc call only can be called by the pool owner
        assert_keys_eq!(
            self.temperature.owner,
            self.owner.key(),
            "pool_owner"
        );
        Ok(())
    }
}

impl ModifyTemperature<'_> {
    pub fn set_pending_owner(
        ctx: Context<ModifyTemperature>,
        new_owner:Pubkey,
    ) -> ProgramResult {
        ctx.accounts.temperature.pending_owner = new_owner;
        Ok(())
    }
    pub fn add_reporter(
        ctx: Context<ModifyTemperature>,
        reporter:Pubkey,
    ) -> ProgramResult{
        ctx.accounts.temperature.reporters.push(Reporter{
            user:reporter,
            last_time:0,
            last_value:0,
            credibility:0,
            active:true
        });
        Ok(())
    }
    pub fn set_reporter_state(
        ctx: Context<ModifyTemperature>,
        index:usize,
        active:bool
    ) -> ProgramResult{
        ctx.accounts.temperature.reporters[index].active = active;
        Ok(())
    }
    pub fn reset_reporter(
        ctx: Context<ModifyTemperature>,
        index:usize,
        reporter:Pubkey
    ) -> ProgramResult{
        ctx.accounts.temperature.reporters[index].user = reporter;
        Ok(())
    }
}

