use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::ConfigGame;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitGame<'info> {
   #[account(mut)]
   pub user: Signer<'info>,
   
   #[account(mint::token_program = token_program)]
   pub mint: InterfaceAccount<'info, Mint>,

   #[account(
      init,
      payer = user,
      space = ConfigGame::DISCRIMINATOR.len() + ConfigGame::INIT_SPACE,
      seeds = [b"config", seed.to_le_bytes().as_ref(), user.key().as_ref(), mint.key().as_ref()],
      bump
   )]
   pub config_game: Account<'info, ConfigGame>,
   
   #[account(
      mut,
      associated_token::token_program = token_program,
      associated_token::authority = user,
      associated_token::mint = mint
   )]
   pub user_ata: InterfaceAccount<'info, TokenAccount>,

   #[account(
      init,
      payer = user,
      associated_token::token_program = token_program,
      associated_token::authority = config_game,
      associated_token::mint = mint
   )]
   pub game_ata: InterfaceAccount<'info, TokenAccount>,

   pub associated_token_program: Program<'info, AssociatedToken>,
   pub token_program: Interface<'info, TokenInterface>,
   pub system_program: Program<'info, System>

}
