use anchor_lang::prelude::*;

declare_id!("Arf2LET7mchncY1Z24eZz71sqoPaFAb1RMtXeUvuET61");

pub const DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter account created. Current count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).ok_or(ErrorCode::Overflow)?;
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous count: {}", counter.count);
        counter.count = counter.count.checked_sub(1).ok_or(ErrorCode::Underflow)?;
        msg!("Counter decremented. Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = user, 
        space = DISCRIMINATOR_SIZE + Counter::INIT_SPACE,
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Math operation overflow")]
    Overflow,
    #[msg("Math operation underflow")]
    Underflow,
}