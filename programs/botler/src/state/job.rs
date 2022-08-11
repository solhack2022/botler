use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct TimebasedJob {
    pub authority: Pubkey,
    pub ix: InstructionData,
    pub status:JobStatus,
}

impl TimebasedJob {

}

pub trait JobAction {
    fn register(
        &mut self,
    ) ->  Result<()>;

    fn execute(
        &mut self,
    ) ->  Result<()>;

    fn cancel(
        &mut self,
    ) ->  Result<()>;
}

impl JobAction for Account<'_, TimebasedJob> {
    fn register() -> Self {
        Self {
            
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Debug)]
pub enum JobStatus {
    Cancelled,
    Executed,
    Queued,
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