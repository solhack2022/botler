use crate::errors::BotlerError;

use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    solana_program::clock::Clock
};

#[derive(Accounts)]
pub struct JobExecute<'info> {
    #[account()]
    pub whitelist: Signer<'info>,
    #[account(
        mut,
        // TODO: add seeds
        seeds=[SEED.as_bytes(), ],
        bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        // TODO: add seeds
        seeds=[SEED.as_bytes(), ],
        bump,
    )]
    pub job: Account<'info, Job>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    /*
        TODO: add more accounts in instruction
    */
}

pub fn handle_job_execute(
    ctx: Context<JobExecute>
) -> Result<()> {
    let whitelist = &ctx.accounts.whitelist;
    let config = &mut ctx.accounts.config;
    let job = &mut ctx.accounts.job;

    require!(
        config.is_whitelisted(&whitelist.key())?,
        BotlerError::JobExecutorNotWhitelisted
    );

    if job.job_type == JobType::Timebased {
        let clock = Clock::get().unwrap().unix_timestamp;
        let schedule = job.schedule.unwrap();
        if (clock >= schedule) {
            job.execute()
        } else {
            BotlerError::JobNotReady
        }
    }
}