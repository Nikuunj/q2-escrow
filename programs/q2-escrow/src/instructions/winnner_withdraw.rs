use anchor_lang::{prelude::*, system_program::{ transfer, Transfer, ID as SYSTEM_PROGRAM_ID}};
use anchor_spl::token_interface::{transfer_checked, TransferChecked, CloseAccount, close_account};

use crate::{CustomError, derive_account::WinnerWithdraw};

impl<'info> WinnerWithdraw<'info> {
   pub fn winner_withdraw(&mut self) -> Result<()> {
      let clock = Clock::get().unwrap();
      let current_time = clock.unix_timestamp;

      let signer_seeds: &[&[&[u8]]] = &[&[
         b"config", 
         &self.config_game.seed.to_le_bytes(),
         self.config_game.create_by.as_ref(),
         self.mint.to_account_info().key.as_ref(), 
         &[self.config_game.bump]
      ]];

      require!(current_time > self.config_game.end_time, CustomError::GameNotEnded);

      let cpi_acc = TransferChecked {
         from: self.game_ata.to_account_info(),
         to: self.winner_ata.to_account_info(),
         authority: self.config_game.to_account_info(),
         mint: self.mint.to_account_info()
      };

      let cpi_ctx = CpiContext::new_with_signer(self.token_program.key(), cpi_acc, signer_seeds);

      transfer_checked(cpi_ctx, self.game_ata.amount, self.mint.decimals).unwrap();

      let cpi_close_acc = CloseAccount {
         account: self.game_ata.to_account_info(),
         authority: self.config_game.to_account_info(),
         destination: self.winner.to_account_info()
      };

      let cpi_close_ctx = CpiContext::new_with_signer(self.token_program.key(), cpi_close_acc, signer_seeds);

      close_account(cpi_close_ctx)

   }
}