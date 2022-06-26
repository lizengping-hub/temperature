use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug,Copy)]
pub struct Reporter {
    pub user:Pubkey,
    pub last_value:u16,
    pub last_time:i64,
    pub credibility:u16,
    pub active:bool
}

