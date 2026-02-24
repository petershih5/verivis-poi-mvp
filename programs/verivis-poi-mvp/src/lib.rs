use anchor_lang::prelude::*;

declare_id!("6hWiksWjhRyopvyszzv4tLLzShJadNTcJG2nZ4mHipc1");

#[program]
pub mod verivis_poi_mvp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
