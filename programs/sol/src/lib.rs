use anchor_lang::prelude::*;

declare_id!("8YLiE4GictjNTKkZUJT8pencgdmNhJMRLZvHe1UadhV5");

#[program]
pub mod sol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
