use anchor_lang::prelude::*;
use anchor_spl::token_interface::{close_account, transfer_checked, CloseAccount, TransferChecked};

use crate::derive_account::Take;

impl<'info> Take<'info> {
    pub fn take(&mut self) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            &self.escrow.seed.to_le_bytes(),
            self.maker.to_account_info().key.as_ref(),
            &self.mint_a.to_account_info().key.as_ref(),
            &[self.escrow.bump],
        ]];

        let transfer_accounts_taker_to_maker_b = TransferChecked {
            mint: self.mint_b.to_account_info(),
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let transfer_account_vault_to_taker_a = TransferChecked {
            mint: self.mint_a.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let transfer_taker_to_maker_b_cpi_ctx = CpiContext::new(
            self.token_program_b.to_account_info().key(),
            transfer_accounts_taker_to_maker_b,
        );
        let transfer_vault_to_taker_a_cpi_ctx = CpiContext::new_with_signer(
            self.token_program_a.to_account_info().key(),
            transfer_account_vault_to_taker_a,
            signer_seeds,
        );

        transfer_checked(
            transfer_taker_to_maker_b_cpi_ctx,
            self.escrow.require,
            self.mint_b.decimals,
        )?;
        transfer_checked(
            transfer_vault_to_taker_a_cpi_ctx,
            self.vault.amount,
            self.mint_a.decimals,
        )?;

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
            destination: self.maker.to_account_info(),
        };

        let close_cpi_ctx = CpiContext::new_with_signer(
            self.token_program_a.to_account_info().key(),
            close_accounts,
            signer_seeds,
        );

        close_account(close_cpi_ctx)
    }
}
