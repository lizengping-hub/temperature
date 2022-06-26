use crate::Post;
use anchor_lang::prelude::*;
use vipers::validate::Validate;
use crate::error::ErrorCode;

impl<'info> Validate<'info> for Post<'info> {
    fn validate(&self) -> ProgramResult {

        Ok(())
    }
}

impl Post<'_> {
    pub fn process(
        ctx: Context<Post>,
        new_amount: u16,
    ) -> ProgramResult {
        let clock = Clock::get()?;
        let opt = ctx.accounts.temperature.find_index_of_reporter(ctx.accounts.reporter.key);
        if opt.is_none()  {
            msg! {"Invalid reporter"};
            return Err(ErrorCode::AccountNotMatch.into());
        }
        ctx.accounts.temperature.post(opt.unwrap(),new_amount)?;
        if clock.unix_timestamp.checked_sub(
            ctx.accounts.temperature.current_round_start).ok_or(ErrorCode::MathOverflow)?
            > ctx.accounts.temperature.round_time {
             ctx.accounts.temperature.new_round()?;
        }
        Ok(())
    }
}

