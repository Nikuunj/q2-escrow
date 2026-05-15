use anchor_lang::prelude::*;

use crate::{derive_account::Init, Escrow, InitBumps};

impl<'info> Init<'info> {
    pub fn init(&mut self, amount: u64, end_time: i64, bumps: &InitBumps) -> Result<()> {
        self.escrow.set_inner(Escrow {
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            require: amount,
            end_time,
            bump: bumps.escrow,
        });
        Ok(())
    }
}
