mod config;
mod graph;
mod cli;
mod hash;
mod cache;
mod execution;

use std::{fs, path::Path};

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init) => {
            println!("Initialisation : création d'un Rustakefile.toml par défaut");
            let file_path = Path::new("Rustakefile.toml");

            // On vérifie si le fichier n'existe pas déjà
            if file_path.exists() {
                println!("Un fichier 'Rustakefile.toml' existe déjà.");
            } else {

                let default_content = r#"# Fichier de configuration Rustake généré automatiquement
                [targets.program]
                depends = ["src/main.rs"]
                command = "rustc src/main.rs -o program"
                "#;
                match fs::write(file_path, default_content) {
                    Ok(_) => println!("Fichier 'Rustakefile.toml' créé avec succès"),
                    Err(e) => eprintln!("Erreur lors de la création du fichier : {}", e),
                }
            }
        }
        Some(Commands::Clean) => {
            println!("Nettoyage : suppression du cache et des cibles");
            let cache_dir = Path::new(".rustake");
            if cache_dir.exists() {
                match fs::remove_dir_all(cache_dir) {
                    Ok(_) => println!("Dossier de cache '.rustake' supprimé."),
                    Err(e) => eprintln!("Erreur lors de la suppression du cache : {}", e),
                }
            } else {
                println!("Aucun cache trouvé.");
            }
            if let Ok(conf) = config::load("Rustakefile.toml") {
                for (name, _) in conf.targets {
                    let target_path = Path::new(&name);
                    if target_path.exists() {
                        // si fichier on supprime
                        match fs::remove_file(target_path) {
                            Ok(_) => println!("Cible '{}' supprimée.", name),
                            Err(e) => eprintln!("Impossible de supprimer la cible '{}' : {}", name, e),
                        }
                    }
                }
            } else {
                println!("Impossible de lire 'Rustakefile.toml', nettoyage des cibles ignoré.");
            }
        }
        None => {
            println!("Démarrage de rustake");

            match config::load("Rustakefile.toml") {
                Ok(conf) => {
                    let dep_graph = graph::DepGraph::from_config(&conf); // on construit le graphe depuis la config fournit
                    let tri_topo = match dep_graph.topogical_sort() {
                        // le tri a marché
                        Ok(tri) => tri,
                        // le tri a detecté un cycle dans les dépendances
                        Err(_) => {
                            eprintln!("Erreur fatale : les dépendances contiennent un cycle");
                            eprintln!("Vérifiez le fichier Rustakefile.toml");
                            std::process::exit(1);
                        }
                    };

                    // on charge le cache et on prépare le tableau des fichiers à compiler
                    let mut cache = cache::Cache::load();
                    let mut commands: Vec<String> = Vec::new();

                    for targ in &tri_topo {
                        // On récupère la target
                        let target = match conf.targets.get(targ) {
                            Some(t) => t,
                            None => continue,
                        };
                        let mut besoin_recomp = false;

                        // On parcourt les dépendances
                        for dep_path in &target.depends {

                            // Si la dépendance n'existe pas on compile
                            if conf.targets.contains_key(dep_path) {
                                let dep_path = Path::new(dep_path);
                                if !dep_path.exists() {
                                    besoin_recomp = true;
                                }
                                continue;
                            }

                            // On regarde le hash
                            match hash::hash_file(Path::new(dep_path)) {
                                Ok(hash_courant) => {
                                    if cache.has_changed(dep_path, &hash_courant) {
                                        println!("{} doit être compilé", dep_path);
                                        besoin_recomp = true;
                                        cache.update_hash(dep_path, hash_courant);
                                    }
                                }
                                Err(_) => {
                                    eprintln!("Erreur fatale : le fichier source {} est introuvable", dep_path);
                                    std::process::exit(1);
                                }
                            }
                        }

                        // Si la target n'existe pas on compile
                        let target_path = Path::new(targ);
                        if !target_path.exists() {
                            println!("{} n'existe pas", targ);
                            besoin_recomp = true;
                        }

                        // On ajoute a la liste de compilation
                        if besoin_recomp {
                            commands.push(target.command.clone());
                        } else {
                            println!("{} est déjà à jour", targ);
                        }
                    }

                    if commands.is_empty() {
                        println!("Tout est à jour");
                    } else {
                        println!("Compilation de {} fichier(s) source(s)", commands.len());
                        if let Err(err) = execution::run_all(&commands) {
                            eprintln!("Erreur pendant l'exécution : {}", err);
                            std::process::exit(1);
                        }

                        if let Err(err) = cache.save() {
                            eprintln!("Impossible de sauvegarder le cache : {}", err);
                        }
                        println!("Compilation terminé");
                    }
                }
                Err(e) => {
                    // Si le fichier n'existe pas ou est mal formaté on quitte
                    eprintln!("Erreur fatale : Impossible de charger 'Rustakefile.toml'.");
                    eprintln!("Détails de l'erreur : {}", e);
                    eprintln!("Astuce : Utilisez `rustake init` pour créer un fichier de base.");
                    std::process::exit(1);
                }
            }

        }
    }
}
