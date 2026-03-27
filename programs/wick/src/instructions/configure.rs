use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::WickError;

#[derive(Accounts)]
pub struct Configure<'info> {
    #[account(
        mut,
        seeds = [Vault::SEED_PREFIX, vault.mint.as_ref()],
        bump = vault.bump,
        has_one = authority @ WickError::Unauthorized,
    )]
    pub vault: Account<'info, Vault>,

    pub authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<Configure>,
    interval_seconds: Option<u64>,
    retention_ratio: Option<u16>,
    min_balance: Option<u64>,
    max_slippage_bps: Option<u16>,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    if let Some(interval) = interval_seconds {
        require!(interval >= 60, WickError::IntervalTooShort);
        vault.interval_seconds = interval;
    }

    if let Some(retention) = retention_ratio {
        require!(retention <= 10000, WickError::InvalidRetentionRatio);
        vault.retention_ratio = retention;
    }

    if let Some(balance) = min_balance {
        vault.min_balance = balance;
    }

    if let Some(slippage) = max_slippage_bps {
        require!(slippage <= 10000, WickError::SlippageExceeded);
        vault.max_slippage_bps = slippage;
    }

    msg!("Vault configured");
    Ok(())
}
