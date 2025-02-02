use anchor_lang::prelude::*;

declare_id!("D1zMUVYxgrhAR9MSKbG464w5GC1fkeuJAG4Ks3wtosoc");

#[program]
pub mod hello {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
