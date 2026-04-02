use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct KeeperConfig {
    pub vault: VaultConfig,
    pub keeper: KeeperSettings,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VaultConfig {
    pub mint: String,
    pub authority: String,
    pub interval_seconds: u64,
    pub min_balance_sol: f64,
    pub retention_ratio: f64,
    pub max_slippage_bps: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeeperSettings {
    pub rpc_url: String,
    pub priority_fee_lamports: u64,
    pub retry_attempts: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub log_to_file: bool,
    pub log_path: String,
}

impl KeeperConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: KeeperConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
