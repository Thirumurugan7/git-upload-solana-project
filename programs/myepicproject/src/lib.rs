use anchor_lang::prelude::*;

declare_id!("9bfQHtHnfRYHXzxAvokjw3u6sq73BsCZx1eYBENY4S8p");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
