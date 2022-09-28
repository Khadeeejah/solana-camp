use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("DrcEfafDtNHtQ8WKoNLtcuQQtmzYbpqiH5ozGBKjJR2V");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff {}
