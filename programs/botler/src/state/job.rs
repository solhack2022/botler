use anchor_lang::{ prelude::*, solana_program::instruction::Instruction };

#[account]
#[derive(Default, Debug)]
pub struct Job {
    pub authority: Pubkey,
    pub ix: Instruction,
    pub status: JobStatus,
    pub job_type: JobType,
    pub schedule: Option<String>,
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
        schedule: Option<String>,
    ) ->  Result<()>;

    fn execute(
        &mut self,
    ) ->  Result<()>;

    fn cancel(
        &mut self,
    ) ->  Result<()>;
}

impl JobAction for Account<'_, Job> {
    fn new(&mut self, authority: Pubkey, ix: Instruction, job_type: JobType, schedule: Option<String>) -> Result<()> {
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
    /* 
        TODO: create fn execute, fn cancel
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