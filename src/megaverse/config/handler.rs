use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Read;
use std::{error, fs::File};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AppProtocol {
    Http,
    #[default]
    Https,
}

impl fmt::Display for AppProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowercase_name = format!("{:?}", self).trim().to_lowercase();
        write!(f, "{}", lowercase_name)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Resources {
    pub polyanets: String,
    pub comeths: String,
    pub goal: String,
    pub soloons: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Config {
    pub host: String,
    pub protocol: AppProtocol,
    pub api_endpoint: String,
    pub port: u32,
    pub parallel: bool,
    pub candidate_id: String,
    pub resources: Resources,
}

pub fn read_config_from_file(filepath: &str) -> Result<Config> {
    debug!("Reading configuration from {filepath}");
    let mut file = File::open(filepath)?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;

    Ok(serde_json::from_str(&data)?)
}
