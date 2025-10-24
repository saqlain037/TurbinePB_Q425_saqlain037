use anchor_lang::prelude::*;

declare_id!("5wf53umhwWgHbjpJxz6ViSh3cf5u8TXVZs8NV17wRcFq");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;
pub const PUBKEY_SIZE: usize = 32;
pub const COUNTER_SIZE: usize = ANCHOR_DISCRIMINATOR_SIZE + 8 + PUBKEY_SIZE;

#[program]
pub mod solana_counter {
    use super::*;

 pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("solana Counter Program! ");

        let counter = &mut ctx.accounts.counter;
        counter.count = 0;

        msg!(
            "The counter has been initialized with a count of {}",
            counter.count
        );
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>, value: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += value;

        msg!(
            "The counter has been incremented by {} to {}",
            value,
            counter.count
        );
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>, value: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(counter.count >= value, ErrorCode::Underflow);
        counter.count -= value;

        msg!(
            "The counter has been decremented by {} to {}",
            value,
            counter.count
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR_SIZE + 8,
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The value of counter can't be negative")]
    Underflow,
}