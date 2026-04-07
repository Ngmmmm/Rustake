use std::fs;
use serde::Deserialize;
use std::error::Error;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub targets: HashMap<String, Target>,
}

#[derive(Deserialize, Debug)]
pub struct Target {
    pub depends: Vec<String>,
    pub command: String,
}

pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let conf: Config = toml::from_str(&content)?;
    Ok(conf)
}
