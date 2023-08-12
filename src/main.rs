#![feature(step_trait)]
mod megaverse;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

use clap::Parser;
use log::{self, debug};
use megaverse::config::handler::{read_config_from_file, Config};
use megaverse::utils::fs::get_abs_path;
use std::error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 2u32)]
    phase: u32,
    #[arg(short, long, default_value_t=String::from("./config.json"))]
    config_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    env_logger::init();
    debug!("Logging engine... Enabled!");

    let filepath = get_abs_path(&args.config_file)?;

    let cfg: Config = match read_config_from_file(&filepath) {
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!(
                "Error reading config from {filepath:?}: {errdescription}",
                errdescription = e.to_string()
            );
            return Err(e);
        }
    };

    debug!("Running configuration is: {cfg:?}");

    if cfg.parallel {
        unimplemented!("Asyncronous interaction towards the client API is not implemented yet.")
    }

    match args.phase {
        1 => {
            megaverse::phases::phase1(&cfg)?;
        }
        2 => {
            megaverse::phases::phase2(&cfg)?;
        }
        _ => {
            unimplemented!();
        }
    }

    Ok(())
}
