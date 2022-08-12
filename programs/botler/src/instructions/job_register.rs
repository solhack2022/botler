use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::{system_program, system_instruction::transfer, program::invoke}},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(ix:InstructionData, schedule: String, amount: u64)]
pub struct TimebasedJobRegister<'info> {
    #[account()]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        // TODO: add seeds
        seeds=[SEED.as_bytes(), ],
        bump,
        payer = payer,
        space = 8 + size_of::<Job>() + schedule.len(),
    )]
    pub job: Account<'info, Job>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handle_timebased_job_register(
    ctx: Context<TimebasedJobRegister>,
    ix: Instruction,
    schedule: String,
    amount: u64,
) -> Result<()> {
    let authority = &ctx.accounts.authority;
    let job = &mut ctx.accounts.job;
    job.new(authority.key(), ix, JobType::Timebased, Some(schedule))?;

    let transfer_ix = transfer(
        &ctx.accounts.payer.key(),
        job.key(),
        amount,
    );
    invoke(
        &transfer_ix,
        &[
            ctx.accounts.payer.to_account_info(),
            job.to_account_info(),
        ],
    )?;

    Ok(())
}