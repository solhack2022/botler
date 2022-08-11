use anchor_lang::{prelude::*, solana_program::system_program};

#[derive(Accounts)]
#[instruction(ix:InstructionData)]
pub struct JobRegister<'info> {

}

pub fn handle_job_register(
    ctx: Context<JobRegister>,
    ix: InstructionData,
) -> Result<()> {
    Ok(())
}