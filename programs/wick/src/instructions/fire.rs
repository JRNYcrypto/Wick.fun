use anchor_lang::prelude::*;
use crate::state::{Vault, WickRecord};
use crate::errors::WickError;

#[derive(Accounts)]
pub struct Fire<'info> {
    #[account(
        mut,
        seeds = [Vault::SEED_PREFIX, vault.mint.as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = keeper,
        space = WickRecord::SPACE,
        seeds = [
            WickRecord::SEED_PREFIX,
            vault.key().as_ref(),
            &vault.total_fires.checked_add(1).unwrap().to_le_bytes(),
        ],
        bump
    )]
    pub wick_record: Account<'info, WickRecord>,

    /// CHECK: keeper is permissionless, any caller can fire the vault
    #[account(mut)]
    pub keeper: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Fire>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;

    let vault_lamports = vault.to_account_info().lamports();
    require!(
        vault.is_ready_to_fire(clock.unix_timestamp, vault_lamports),
        WickError::TimerNotElapsed
    );

    // 1. compute buy amount (all available vault lamports minus rent-exempt minimum)
    let rent = Rent::get()?;
    let rent_exempt = rent.minimum_balance(Vault::SPACE);
    let buy_lamports = vault_lamports
        .checked_sub(rent_exempt)
        .ok_or(WickError::MathOverflow)?;

    // 2. execute atomic buy (CPI to swap program, e.g. pumpswap or jupiter)
    //    the actual swap CPI is omitted here — integration happens in the keeper

    // 3. compute sell amount from retention ratio
    let sell_ratio = 10000u64
        .checked_sub(vault.retention_ratio as u64)
        .ok_or(WickError::MathOverflow)?;

    // tokens_bought would come back from the buy CPI result
    let tokens_bought: u64 = 0; // placeholder, set by CPI return
    let tokens_sold = tokens_bought
        .checked_mul(sell_ratio)
        .ok_or(WickError::MathOverflow)?
        .checked_div(10000)
        .ok_or(WickError::MathOverflow)?;

    let tokens_retained = tokens_bought
        .checked_sub(tokens_sold)
        .ok_or(WickError::MathOverflow)?;

    // 4. execute atomic sell (CPI to swap program)

    // 5. record the wick
    let record = &mut ctx.accounts.wick_record;
    record.vault = vault.key();
    record.index = vault.total_fires.checked_add(1).ok_or(WickError::MathOverflow)?;
    record.fired_at = clock.unix_timestamp;
    record.sol_spent = buy_lamports;
    record.tokens_bought = tokens_bought;
    record.tokens_sold = tokens_sold;
    record.tokens_retained = tokens_retained;
    record.sol_received = 0; // placeholder, set by sell CPI return
    record.net_volume = buy_lamports
        .checked_add(record.sol_received)
        .ok_or(WickError::MathOverflow)?;

    // 6. update vault stats
    vault.last_fired_at = clock.unix_timestamp;
    vault.total_fires = record.index;
    vault.total_volume_sol = vault.total_volume_sol
        .checked_add(record.net_volume)
        .ok_or(WickError::MathOverflow)?;
    vault.total_tokens_retained = vault.total_tokens_retained
        .checked_add(tokens_retained)
        .ok_or(WickError::MathOverflow)?;

    msg!("Wick #{} fired. Volume: {} lamports", record.index, record.net_volume);
    Ok(())
}
