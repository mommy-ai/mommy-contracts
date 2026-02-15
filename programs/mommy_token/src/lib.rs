use anchor_lang::prelude::*;

declare_id!("MoMMYAi111111111111111111111111111111111111");

#[program]
pub mod mommy_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("🤱 MOMMY Token initialized!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
