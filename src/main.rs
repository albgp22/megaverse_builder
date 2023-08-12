#![feature(step_trait)]
mod megaverse;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

use log::{self, debug};
use megaverse::config::handler::{read_config_from_file, Config};
use megaverse::utils::fs::get_abs_path;
use std::error;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    debug!("Logging engine... Enabled!");

    let filepath = get_abs_path("./config.json")?;

    let cfg: Config = match read_config_from_file(&filepath) {
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!(
                "Error reading {filepath:?}: {errdescription}",
                errdescription = e.to_string()
            );
            return Err(e);
        }
    };

    debug!("Running configuration is: {cfg:?}");

    if cfg.parallel {
        unimplemented!("Asyncronous interaction towards the client API is not implemented yet.")
    }

    megaverse::phases::phase1(&cfg)?;

    Ok(())
}
