use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VaultInfo {
    pub address: String,
    pub mint: String,
    pub authority: String,
    pub balance_lamports: u64,
    pub interval_seconds: u64,
    pub retention_ratio: u16,
    pub last_fired_at: i64,
    pub total_fires: u64,
    pub total_volume_sol: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WickEntry {
    pub index: u64,
    pub fired_at: i64,
    pub volume_sol: f64,
    pub tokens_retained: u64,
    pub tx_signature: String,
}

pub struct WickClient {
    pub rpc_url: String,
}

impl WickClient {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
        }
    }

    pub async fn get_vault(&self, mint: &str) -> Result<VaultInfo, Box<dyn std::error::Error>> {
        // derive vault PDA from mint, fetch account, deserialize
        let _ = mint;
        Ok(VaultInfo::default())
    }

    pub async fn get_wick_history(
        &self,
        mint: &str,
        limit: u8,
    ) -> Result<Vec<WickEntry>, Box<dyn std::error::Error>> {
        // fetch wick records for vault
        let _ = (mint, limit);
        Ok(vec![])
    }
}
