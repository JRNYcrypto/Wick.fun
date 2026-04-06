mod config;
mod executor;
mod logger;
mod monitor;

use config::KeeperConfig;
use executor::WickExecutor;
use logger::WickLogger;
use monitor::VaultMonitor;
use log::{error, info};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("wick keeper starting");

    let config = KeeperConfig::load("config.yaml")?;
    info!("loaded config for vault mint {}", config.vault.mint);

    let monitor = VaultMonitor::new(&config);
    let executor = WickExecutor::new(&config);
    let logger = WickLogger::new(&config);

    let interval = Duration::from_secs(10);

    loop {
        match monitor.check_vault().await {
            Ok(status) => {
                if status.ready {
                    info!("vault ready to fire, balance {} SOL", status.balance_sol);
                    match executor.fire_wick().await {
                        Ok(result) => {
                            info!("wick fired: {:?}", result);
                            if let Err(e) = logger.log_wick(&result).await {
                                error!("failed to log wick: {}", e);
                            }
                        }
                        Err(e) => error!("fire failed: {}", e),
                    }
                } else {
                    info!(
                        "vault not ready. balance: {} SOL, next fire in {}s",
                        status.balance_sol, status.seconds_until_fire
                    );
                }
            }
            Err(e) => error!("monitor check failed: {}", e),
        }

        tokio::time::sleep(interval).await;
    }
}
