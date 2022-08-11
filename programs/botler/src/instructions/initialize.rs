use crate::state::*;
use crate::constants;

use anchor_lang::prelude::*;
use solana_program::system_program;
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct InitializeBotler<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds=[SEED.as_bytes(), ],
        bump,
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pun fn handle_initialize_botler(
    ctx: Context<InitializeBotler>,
    bump: u8,
) -> Result<()> {
    
    Ok(())
}