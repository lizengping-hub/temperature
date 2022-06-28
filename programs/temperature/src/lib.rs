pub mod state;
mod processor;
mod error;
mod math;

use vipers::validate::Validate;
use anchor_lang::prelude::*;
use crate::state::temperature::Temperature;
declare_id!("66TSa2MG2MMzYSesUAwKdf5SZ72wteTY1En1bzVNC9r1"); // production program id

#[program]
pub mod temperature {
    use super::*;

    ///
    #[access_control(ctx.accounts.validate())]
    pub fn create_temperature(ctx: Context<CreateTemperature>) -> ProgramResult {
        CreateTemperature::create_temperature(ctx)
    }
    #[access_control(ctx.accounts.validate())]
    pub fn post(ctx:Context<Post>,new_temperature:u16) ->ProgramResult{
        Post::process(ctx,new_temperature)
    }
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn set_pending_pool(ctx: Context<ModifyTemperature>,
                            new_owner: Pubkey) -> ProgramResult {
        ModifyTemperature::set_pending_owner(ctx, new_owner)
    }

    #[access_control(ctx.accounts.validate())]
    pub fn add_reporter(ctx: Context<ModifyTemperature>,
                        reporter: Pubkey) -> ProgramResult {
        ModifyTemperature::add_reporter(ctx, reporter)
    }

    #[access_control(ctx.accounts.validate())]
    pub fn set_reporter_state(
        ctx: Context<ModifyTemperature>,
        index: usize,
        active: bool) -> ProgramResult {
        ModifyTemperature::set_reporter_state(ctx, index, active)
    }

    #[access_control(ctx.accounts.validate())]
    pub fn reset_reporter(ctx: Context<ModifyTemperature>,
                          index: usize,
                          reporter: Pubkey) -> ProgramResult {
        ModifyTemperature::reset_reporter(ctx, index, reporter)
    }
    ///
    #[access_control(ctx.accounts.validate())]
    pub fn receive_pending_pool(ctx: Context<ReceiveOwner>) -> ProgramResult {
        ReceiveOwner::process(ctx)
    }
}

#[derive(Accounts)]
#[instruction(max_reporter_count: u8)]
pub struct CreateTemperature<'info> {
    #[account(
    init,
    payer = payer,
    space = (8 + 80 + 52 * max_reporter_count as u32 + 4) as usize
    )]
    pub temperature: Account<'info, Temperature>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub owner: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyTemperature<'info> {
    #[account(mut)]
    pub temperature: Account<'info, Temperature>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReceiveOwner<'info> {
    #[account(mut)]
    pub temperature: Account<'info, Temperature>,
    pub pending_owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Post<'info> {
    #[account(mut)]
    pub temperature: Account<'info, Temperature>,
    pub reporter:Signer<'info>,
}