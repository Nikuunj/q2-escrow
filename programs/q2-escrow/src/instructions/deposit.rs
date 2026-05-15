use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{transfer_checked, TransferChecked},
};

use crate::derive_account::Deposit;



impl<'info> Deposit<'info> {

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let transfer_account = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program_a.to_account_info().key(), transfer_account);

        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)
    }
}
