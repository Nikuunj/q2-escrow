use anchor_lang::prelude::*;

#[account]
pub struct ConfigGame {
    pub reciever: Pubkey,
    pub last_amount: u64,
    pub mint: Pubkey,
    pub end_time: i64,
    pub bump: u8,
}

#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct Escrow {
    pub maker: Pubkey,
    pub require: u64,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub end_time: i64,
    pub bump: u8,
}
