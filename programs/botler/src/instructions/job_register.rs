use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program}
};

#[derive(Accounts)]
#[instruction(ix:InstructionData, schedule: String)]
pub struct TimebasedJobRegister<'info> {
    #[account()]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        seeds=[SEED.as_bytes(), ],
        bump,
        payer = payer,
        space = 8 + size_of::<Job>() + schedule.len(),
    )]
    pub job: Account<'info, Job>,
}

pub fn handle_timebased_job_register(
    ctx: Context<TimebasedJobRegister>,
    ix: InstructionData,
    schedule: String,
) -> Result<()> {
    let authority = &ctx.accounts.authority;
    let job = &mut ctx.accounts.job;
    job.register(authority.key(), ix, JobType::Timebased, Some(schedule))
}