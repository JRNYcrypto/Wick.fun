use crate::config::KeeperConfig;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

pub struct VaultStatus {
    pub balance_sol: f64,
    pub ready: bool,
    pub seconds_until_fire: i64,
}

pub struct VaultMonitor {
    pub rpc: RpcClient,
    pub vault_address: String,
    pub min_balance_sol: f64,
    pub interval_seconds: i64,
}

impl VaultMonitor {
    pub fn new(config: &KeeperConfig) -> Self {
        Self {
            rpc: RpcClient::new_with_commitment(
                config.keeper.rpc_url.clone(),
                CommitmentConfig::confirmed(),
            ),
            vault_address: config.vault.authority.clone(),
            min_balance_sol: config.vault.min_balance_sol,
            interval_seconds: config.vault.interval_seconds as i64,
        }
    }

    pub async fn check_vault(&self) -> Result<VaultStatus, Box<dyn std::error::Error>> {
        // fetch vault balance, compute time-until-fire
        // actual implementation derives vault PDA and reads Vault account data
        let balance_sol = 0.0; // placeholder, fetch via rpc.get_balance on vault PDA
        let seconds_until_fire = 0i64; // placeholder, compute from last_fired_at

        Ok(VaultStatus {
            balance_sol,
            ready: balance_sol >= self.min_balance_sol && seconds_until_fire <= 0,
            seconds_until_fire,
        })
    }
}
