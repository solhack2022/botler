use anchor_lang::{prelude::*, solana_program::system_program};

#[derive(Accounts)]
pub struct JobCancel<'info> {

}

pub fn handle_job_cancel(
    ctx: Context<JobCancel>
) -> Result<()> {
    Ok(())
}