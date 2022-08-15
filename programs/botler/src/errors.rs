use anchor_lang::prelude::*;

#[error_code]
pub enum BotlerError {
    #[msg("Invalid instruction")]
    InvalidInstruction,
    #[msg("Job is not ready to execute")]
    JobNotReady,
    #[msg("Job executor is not whitelisted")]
    JobExecutorNotWhitelisted,
    #[msg("Job is not registered")]
    JobNotRegistered,
    #[msg("Instruction execution failed")]
    IxExecutionFailed,
}