use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};

use crate::{ConfigGame, InitGameBumps, derive_account::InitGame, CustomError};

impl<'info> InitGame<'info> {
   pub fn init_game(&mut self, seed: u64, end_time: i64, amount: u64, bumps: &InitGameBumps) -> Result<()> {
      let clock = Clock::get().unwrap();
      let current_time = clock.unix_timestamp;

      require!(amount > 0, CustomError::InvalidAmount);
      require!(end_time >= current_time + 300, CustomError::InvalidEndTime);
      
      let tx_acc = TransferChecked {
         from: self.user_ata.to_account_info(),
         to: self.game_ata.to_account_info(),
         authority: self.user.to_account_info(),
         mint: self.mint.to_account_info()
      };

      let cpi_ctx = CpiContext::new(self.token_program.key(), tx_acc);

      transfer_checked(cpi_ctx, amount, self.mint.decimals).unwrap();

      self.config_game.set_inner(ConfigGame {
         winner: self.user.key(),
         last_deposit_amount: amount,
         mint: self.mint.key(),
         end_time,
         seed,
         bump: bumps.config_game,
         create_by: self.user.key(),
      });

      Ok(())
   }
}