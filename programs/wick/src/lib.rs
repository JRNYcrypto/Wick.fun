use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("WickXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");

#[program]
pub mod wick {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        interval_seconds: u64,
        retention_ratio: u16,
        min_balance: u64,
        max_slippage_bps: u16,
    ) -> Result<()> {
        instructions::initialize::handler(
            ctx,
            interval_seconds,
            retention_ratio,
            min_balance,
            max_slippage_bps,
        )
    }

    pub fn fire(ctx: Context<Fire>) -> Result<()> {
        instructions::fire::handler(ctx)
    }

    pub fn configure(
        ctx: Context<Configure>,
        interval_seconds: Option<u64>,
        retention_ratio: Option<u16>,
        min_balance: Option<u64>,
        max_slippage_bps: Option<u16>,
    ) -> Result<()> {
        instructions::configure::handler(
            ctx,
            interval_seconds,
            retention_ratio,
            min_balance,
            max_slippage_bps,
        )
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, amount)
    }
}
