use crate::{CreateTemperature};
use anchor_lang::prelude::*;
use vipers::validate::Validate;
impl<'info> Validate<'info> for CreateTemperature<'info> {
    fn validate(&self) -> ProgramResult {
        Ok(())
    }
}
impl CreateTemperature<'_> {
    pub fn create_temperature(
        ctx: Context<CreateTemperature>
    ) -> ProgramResult {
        ctx.accounts.temperature.owner = ctx.accounts.owner.key();
        Ok(())
    }
}