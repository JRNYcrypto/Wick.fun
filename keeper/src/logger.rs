use crate::config::KeeperConfig;
use crate::executor::WickResult;
use log::info;
use std::fs::OpenOptions;
use std::io::Write;

pub struct WickLogger {
    pub log_to_file: bool,
    pub log_path: String,
}

impl WickLogger {
    pub fn new(config: &KeeperConfig) -> Self {
        Self {
            log_to_file: config.logging.log_to_file,
            log_path: config.logging.log_path.clone(),
        }
    }

    pub async fn log_wick(&self, result: &WickResult) -> Result<(), Box<dyn std::error::Error>> {
        let entry = format!(
            "wick #{} | vol: {} SOL | bought: {} | sold: {} | retained: {} | tx: {}",
            result.index,
            result.volume_sol,
            result.tokens_received,
            result.tokens_sold,
            result.tokens_retained,
            result.tx_signature,
        );

        info!("{}", entry);

        if self.log_to_file {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.log_path)?;
            writeln!(file, "{}", entry)?;
        }

        Ok(())
    }
}
