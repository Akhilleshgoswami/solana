use anchor_lang::prelude::*;

declare_id!("93LiZ984KXxiqNbwkSrjY9dPs36F5EcprYsRpZY14PVm");

#[program]
pub mod init {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
