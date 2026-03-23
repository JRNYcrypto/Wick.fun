use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,

    // timing
    pub interval_seconds: u64,
    pub last_fired_at: i64,
    pub created_at: i64,

    // economics
    pub retention_ratio: u16,        // basis points, 2000 = 20%
    pub min_balance: u64,            // lamports
    pub max_slippage_bps: u16,

    // stats
    pub total_fires: u64,
    pub total_volume_sol: u64,
    pub total_tokens_retained: u64,
}

impl Vault {
    pub const SEED_PREFIX: &'static [u8] = b"vault";
    pub const SPACE: usize = 8 + 32 + 32 + 1 + 8 + 8 + 8 + 2 + 8 + 2 + 8 + 8 + 8;

    pub fn is_ready_to_fire(&self, current_time: i64, current_balance: u64) -> bool {
        let time_elapsed = current_time - self.last_fired_at;
        time_elapsed >= self.interval_seconds as i64 && current_balance >= self.min_balance
    }
}

#[account]
pub struct WickRecord {
    pub vault: Pubkey,
    pub index: u64,
    pub fired_at: i64,
    pub sol_spent: u64,
    pub tokens_bought: u64,
    pub tokens_sold: u64,
    pub tokens_retained: u64,
    pub sol_received: u64,
    pub net_volume: u64,
}

impl WickRecord {
    pub const SEED_PREFIX: &'static [u8] = b"wick";
    pub const SPACE: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8;
}
