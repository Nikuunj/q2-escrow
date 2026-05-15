use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::states::Escrow;

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(mint::token_program = token_program_a)]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program_b)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = maker,
        space = Escrow::DISCRIMINATOR.len() + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), mint_a.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    pub token_program_a: Interface<'info, TokenInterface>,
    pub token_program_b: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}