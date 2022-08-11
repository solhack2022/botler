use anchor_lang::prelude::*;

#[error_code]
pub enum BotlerError {
    #[msg("Invalid instruction")]
    InvalidInstruction,
}