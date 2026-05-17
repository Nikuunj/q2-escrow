use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};

use crate::{CustomError, derive_account::BeingWinner};

impl <'info> BeingWinner<'info> {
   pub fn being_winner(&mut self, amount: u64) -> Result<()> {

      let clock = Clock::get().unwrap();
      let current_time = clock.unix_timestamp;

      // let signer_seeds: &[&[&[u8]]] = &[&[
      //    b"config", 
      //    self.config_game.seed.to_le_bytes().as_ref(),
      //    self.config_game.create_by.as_ref(),
      //    self.mint.key().as_ref(), 
      //    &[self.config_game.bump]
      // ]];

      require!(amount > self.config_game.last_deposit_amount, CustomError::InvalidAmount);
      require!(current_time < self.config_game.end_time, CustomError::Timeout);

      let cpi_acc = TransferChecked {
         from: self.player_ata.to_account_info(),
         to: self.game_ata.to_account_info(),
         mint: self.mint.to_account_info(),
         authority: self.player.to_account_info()
      };

      let cpi_ctx = CpiContext::new(self.token_program.key(), cpi_acc);

      self.config_game.last_deposit_amount = amount;
      self.config_game.winner = self.player.key();
      transfer_checked(cpi_ctx, amount, self.mint.decimals)
   }
}