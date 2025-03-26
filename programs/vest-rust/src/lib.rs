use anchor_lang::prelude::*;

declare_id!("Cni9tndj9ApM86C69SugKKFmMFSHQLASCqrfWxn5umg1");

#[program]
pub mod vest_rust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
