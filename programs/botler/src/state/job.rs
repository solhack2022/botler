use anchor_lang::{ prelude::*, solana_program::{instruction::Instruction, program::invoke_signed} };
use crate::errors::BotlerError;

#[account]
#[derive(Default, Debug)]
pub struct Job {
    pub authority: Pubkey,
    pub ix: Instruction,
    pub status: JobStatus,
    pub job_type: JobType,
    pub schedule: Option<u64>,
}

impl Job {
    /* 
        TODO:
    */
}

pub trait JobAction {
    fn new(
        &mut self,
        authority: Pubkey,
        ix: Instruction,
        job_type: JobType,
        schedule: Option<u64>,
    ) ->  Result<()>;

    fn execute(
        &mut self,
    ) ->  Result<()>;

    fn cancel(
        &mut self,
    ) ->  Result<()>;
}

impl JobAction for Account<'_, Job> {
    fn new(&mut self, authority: Pubkey, ix: Instruction, job_type: JobType, schedule: Option<u64>) -> Result<()> {
        /* 
            TODO: add require to check if accountMetadata is valid
        */
        self.authority = authority.key();
        self.ix = ix;
        self.status = JobStatus::Registered;
        self.job_type = job_type;
        self.schedule = schedule;

        Ok(())
    }
    
    fn execute(
            &mut self,
        ) ->  Result<()> {
            require!(self.status == JobStatus::Registered, BotlerError::JobNotRegistered);
            invoke_signed(
                instruction: &self.ix, 
                account_infos, signers_seeds
            )
                .map_err(|e| BotlerError::IxExecutionFailed)?;
            Ok(())
    }

    /* 
        TODO: create fn cancel
    */
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum JobStatus {
    Registered,
    Cancelled,
    Executed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum JobType {
    Timebased,
    Conditional,
    Both,
}