use anchor_lang::prelude::*;

declare_id!("Dt8EjSDu8DQTUqcY11ksFQ1VidUUb6HdfBbR7msGExTK");

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
