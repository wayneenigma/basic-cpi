use anchor_lang::prelude::*;

declare_id!("Bxi8KcfKcCDA6n8gmyZaHSfpXBBSjuTZTxsgfmdSBU1g");

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
    pub system_program: Program<'info, System>
}
