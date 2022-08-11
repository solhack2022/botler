use crate::state;

use anchor_lang::prelude::*;
use solana_program::system_program;

#[derive(Accounts)]
#[instruction(bump:u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds=[],
        bump,
        payer = signer,
        space = 
    )]
    pub authority: Account<'info, Authority>,
}

pun fn handle_initialize_botler(
    ctx: Context<Initialize>
) -> Result<()> {
    
    Ok(())
}