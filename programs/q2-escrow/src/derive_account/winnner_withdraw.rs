use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::ConfigGame;

#[derive(Accounts)]
pub struct WinnerWithdraw<'info> {
   #[account(mut)]
   pub winner: Signer<'info>,

   #[account(mint::token_program = token_program)]
   pub mint: InterfaceAccount<'info, Mint>,

   #[account(
      mut,
      has_one = mint,
      has_one = winner,
      close = winner,
      seeds = [b"config", config_game.seed.to_le_bytes().as_ref(), config_game.create_by.as_ref(), mint.key().as_ref()],
      bump = config_game.bump
   )]
   pub config_game: Account<'info, ConfigGame>,
   
   #[account(
      mut,
      associated_token::token_program = token_program,
      associated_token::authority = winner,
      associated_token::mint = mint
   )]
   pub winner_ata: InterfaceAccount<'info, TokenAccount>,

   #[account(
      mut,
      associated_token::token_program = token_program,
      associated_token::authority = config_game,
      associated_token::mint = mint
   )]
   pub game_ata: InterfaceAccount<'info, TokenAccount>,

   pub associated_token_program: Program<'info, AssociatedToken>,
   pub token_program: Interface<'info, TokenInterface>,
   pub system_program: Program<'info, System>
}