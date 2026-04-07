use std::collections::HashMap;
use crate::config::Config;

pub struct DepGraph {
    // String = Noeud, Vec<String> = fils du noeud
    pub dependencies: HashMap<String, Vec<String>>,
}

impl DepGraph {
    pub fn from_config(config: &Config) -> Self {
	let mut deps: HashMap<String, Vec<String>> = HashMap::new();

	for (name, cfg) in &config.targets {

	    // On regarde si un noeud 'name' existe, or_default() fait un Vec<String> vide si le noeud est pas la
	    // On rempli le Vec<String> avec extend
	    deps
		.entry(name.clone()) 
		.or_default()
		.extend(cfg.depends.iter().cloned());

	    // On vérifie que les noeuds existent
	    for dep in &cfg.depends {
		deps.entry(dep.clone()).or_default();
	    }
	}

	DepGraph { dependencies: deps }
    }
}
