use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::WickError;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Vault::SPACE,
        seeds = [Vault::SEED_PREFIX, mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    /// CHECK: token mint, validated by seeds
    pub mint: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    interval_seconds: u64,
    retention_ratio: u16,
    min_balance: u64,
    max_slippage_bps: u16,
) -> Result<()> {
    require!(interval_seconds >= 60, WickError::IntervalTooShort);
    require!(retention_ratio <= 10000, WickError::InvalidRetentionRatio);
    require!(max_slippage_bps <= 10000, WickError::SlippageExceeded);

    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;

    vault.authority = ctx.accounts.authority.key();
    vault.mint = ctx.accounts.mint.key();
    vault.bump = ctx.bumps.vault;

    vault.interval_seconds = interval_seconds;
    vault.last_fired_at = clock.unix_timestamp;
    vault.created_at = clock.unix_timestamp;

    vault.retention_ratio = retention_ratio;
    vault.min_balance = min_balance;
    vault.max_slippage_bps = max_slippage_bps;

    vault.total_fires = 0;
    vault.total_volume_sol = 0;
    vault.total_tokens_retained = 0;

    msg!("Wick vault initialized for mint {}", vault.mint);
    msg!("Interval: {}s, retention: {}bps, min balance: {} lamports",
        interval_seconds, retention_ratio, min_balance);

    Ok(())
}
