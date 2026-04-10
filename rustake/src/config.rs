use std::fs;
use serde::Deserialize;
use std::error::Error;
use std::collections::HashMap;

///Permet de contenir la listes des cibles du fichier de configuration en associant le nom des cibles à leur contenu
#[derive(Deserialize, Debug)]
pub struct Config {
    pub targets: HashMap<String, Target>,
}

///Permet de contenir les informations d'une des cibles du fichier de configuration
#[derive(Deserialize, Debug)]
pub struct Target {
    pub depends: Vec<String>,
    pub command: String,
}

///Permet de charger une configuration depuis le fichier de configuration fournit dans 'PATH'
///RETURN VALUE: Result: - OK = Une configuration fonctionnelle qui contient les informations du .toml
///                      - Err = Les erreurs de read_to_string ou from_str qui sont renvoyés par propagation.
pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let conf: Config = toml::from_str(&content)?;
    Ok(conf)
}
