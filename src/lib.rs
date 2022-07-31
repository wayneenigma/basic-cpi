use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod basic {
    use super::*;
    pub fn init(_ctx: Context<Init>) -> Result<()> {
        msg!("initialized");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    pub user: Signer<'info>,
}
