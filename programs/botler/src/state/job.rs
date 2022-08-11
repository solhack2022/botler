use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct Job {
    pub authority: Pubkey,
    pub ix: InstructionData,
    pub status: JobStatus,
    pub job_type: JobType,
    pub schedule: Option<String>,
}

impl Job {

}

pub trait JobAction {
    fn register(
        &mut self,
        authority: Pubkey,
        ix: InstructionData,
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
    fn register(&mut self, authority: Pubkey, ix: InstructionData, job_type: JobType, schedule: Option<String>) -> Result<()> {
        self.authority = authority.key();
        self.ix = ix;
        self.status = JobStatus::Registered;
        self.job_type = job_type;
        self.schedule = schedule;

        Ok(())
    }
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

#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct InstructionData {
    /// Pubkey of the instruction processor that executes this instruction
    pub program_id: Pubkey,
    /// Metadata for what accounts should be passed to the instruction processor
    pub accounts: Vec<AccountMetaData>,
    /// Opaque data passed to the instruction processor
    pub data: Vec<u8>,
}
