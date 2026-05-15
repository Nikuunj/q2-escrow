use anchor_lang::prelude::*;

declare_id!("DvbCYtSnXwdC49d5rsLkeRjVLxtwSRYuN8Y1TZzteeL9");

#[program]
pub mod q2_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
