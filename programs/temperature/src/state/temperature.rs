use anchor_lang::prelude::*;
use crate::error::ErrorCode;
use crate::math::{Decimal, TryDiv, TrySub};
use crate::state::reporter::Reporter;

#[account]
#[derive(Default, Debug)]
pub struct Temperature {
    pub owner:Pubkey,
    pub pending_owner:Pubkey,
    pub decimal:u8,
    pub round_time:i64,
    pub current_round_start:i64,
    pub report_count:u32,
    pub value:u32,
    pub valid_report_count:u32,
    pub reporters: Vec<Reporter>
}

impl Temperature {
    pub fn find_index_of_reporter(&self,reporter_pubkey:&Pubkey)->Option<usize>{
        self.reporters.iter().position(|&reporter| reporter.user.eq(reporter_pubkey))
    }
    pub fn post(&mut self,index:usize,new_amount:u16)->Result<usize,ProgramError>{
        self.reporters[index].last_value = new_amount;
        self.reporters[index].last_time = Clock::get()?.unix_timestamp;
        Ok(index)
    }
    pub fn new_round(&mut self)->Result<(),ProgramError>{
        let available_sorted_reporter = self.get_available_sorted_reporter()?;
        let middle_index = available_sorted_reporter.len().checked_div(0).ok_or(ErrorCode::MathOverflow)?;
        let middle_value = Decimal::from(available_sorted_reporter[middle_index].last_value as u64);
        let mut sum:u64 = 0;
        let mut count = 0;

        let min_diff = Decimal::from(1).try_div(100u64)?;
        for i in 0..available_sorted_reporter.len() {
            let reporter = self.reporters[i];
            let current_value = Decimal::from(reporter.last_value as u64);
            let current_diff = Self::abs(current_value,middle_value)?.try_div(middle_value)?;
            if current_diff.lt(&min_diff){
                sum = sum.checked_add(reporter.last_value as u64).ok_or(ErrorCode::MathOverflow)?;
                count = count+1;
            }
        }
        let average = sum.checked_div(count).ok_or(ErrorCode::MathOverflow)?;
        self.value = average as u32;
        self.current_round_start = Clock::get()?.unix_timestamp;
        Ok(())
    }
    fn abs(a:Decimal,b:Decimal)->Result<Decimal,ProgramError>{
        return if a.gt(&b) {
            a.try_sub(b)
        } else {
            b.try_sub(a)
        }
    }
    fn get_available_sorted_reporter(&self)->Result<Vec<Reporter>,ProgramError>{
        let mut available_sorted:Vec<Reporter> = Vec::new();
        for i in 0.. self.reporters.len() {
            let reporter = self.reporters[i];
            if reporter.last_time > self.current_round_start{
                for j in 0..available_sorted.len(){
                    if available_sorted[j].last_value > reporter.last_value{
                        available_sorted.insert(j, reporter);
                        break
                    }
                    if j == available_sorted.len()-1{
                        available_sorted.insert(j, reporter);
                    }
                }
            }
        }
        Ok(available_sorted)
    }
}