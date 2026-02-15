use anchor_lang::prelude::*;

declare_id!("MoMMyAi111111111111111111111111111111111111");

#[program]
pub mod mommy_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("MOMMY Token initialized 💛");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
