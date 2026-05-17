use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct ConfigGame {
    pub winner: Pubkey,
    pub last_deposit_amount: u64,
    pub mint: Pubkey,
    pub end_time: i64,
    pub seed: u64,
    pub bump: u8,
    pub create_by: Pubkey
}

#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct Escrow {
    pub maker: Pubkey,
    pub require: u64,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub seed: u64,
    pub end_time: i64,
    pub bump: u8,
}
