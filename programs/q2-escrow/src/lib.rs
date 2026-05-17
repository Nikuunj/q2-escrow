use anchor_lang::prelude::*;

declare_id!("DvbCYtSnXwdC49d5rsLkeRjVLxtwSRYuN8Y1TZzteeL9");

mod derive_account;
mod states;
mod instructions;
mod errors;

pub use derive_account::*;
pub use states::*;
pub use errors::*;

#[program]
pub mod q2_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Init>, seed: u64, amount: u64, end_time: i64) -> Result<()> {
        ctx.accounts.init(seed, amount, end_time, &ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.take()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()
    }
}