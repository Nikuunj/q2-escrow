use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub maker: Signer<'info>,
}
