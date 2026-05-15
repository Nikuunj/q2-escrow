use anchor_lang::prelude::*;
use anchor_spl::token_interface::{close_account, transfer_checked, CloseAccount, TransferChecked};

use crate::derive_account::Refund;

impl<'info> Refund<'info> {
    pub fn refund(&mut self) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.mint_a.to_account_info().key.as_ref(),
            &[self.escrow.bump],
        ]];

        let transfer_account_vault_to_maker = TransferChecked {
            mint: self.mint_a.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let transfer_vault_to_maker_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info().key(),
            transfer_account_vault_to_maker,
            signer_seeds,
        );

        transfer_checked(
            transfer_vault_to_maker_cpi_ctx,
            self.vault.amount,
            self.mint_a.decimals,
        )?;

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
            destination: self.maker.to_account_info(),
        };

        let close_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info().key(),
            close_accounts,
            signer_seeds,
        );

        close_account(close_cpi_ctx)
    }
}
