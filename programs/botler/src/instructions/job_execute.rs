use anchor_lang::{prelude::*, solana_program::system_program};

#[derive(Accounts)]
pub struct JobExecute<'info> {

}

pub fn handle_job_execute(
    ctx: Context<JobExecute>
) -> Result<()> {
    Ok(())
}