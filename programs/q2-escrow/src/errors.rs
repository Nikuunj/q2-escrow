use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
   #[msg("Invalid Amount")]
   InvalidAmount,
   #[msg("Invalid End Time")]
   InvalidEndTime,
   #[msg("Timeout for participent in this Game")]
   Timeout, 
   #[msg("Wait for gamed")]
   GameNotEnded, 
}