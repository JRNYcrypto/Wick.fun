use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::errors::WickError;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [Vault::SEED_PREFIX, vault.mint.as_ref()],
        bump = vault.bump,
        has_one = authority @ WickError::Unauthorized,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let vault_info = ctx.accounts.vault.to_account_info();
    let authority_info = ctx.accounts.authority.to_account_info();

    let vault_balance = vault_info.lamports();
    let rent = Rent::get()?;
    let rent_exempt = rent.minimum_balance(Vault::SPACE);
    let available = vault_balance
        .checked_sub(rent_exempt)
        .ok_or(WickError::MathOverflow)?;

    require!(amount <= available, WickError::InsufficientBalance);

    **vault_info.try_borrow_mut_lamports()? = vault_balance
        .checked_sub(amount)
        .ok_or(WickError::MathOverflow)?;
    **authority_info.try_borrow_mut_lamports()? = authority_info
        .lamports()
        .checked_add(amount)
        .ok_or(WickError::MathOverflow)?;

    msg!("Withdrew {} lamports from vault", amount);
    Ok(())
}
