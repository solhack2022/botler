pub mod errors;
pub mod state;
pub mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod botler {
    use super::*;
    pub fn initialize_botler(ctx: Context<Initialize>) -> Result<()> {
        handle_initialize_botler(ctx)
    }
}

