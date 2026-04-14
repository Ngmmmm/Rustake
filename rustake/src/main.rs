mod config;
mod graph;
mod cli;

use std::{fs, path::Path};

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init) => {
            println!("Initialisation : création d'un Rustakefile.toml par défaut");
            // TODO: Logique pour créer le fichier par défaut
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
            // TODO: Logique pour supprimer le dossier .rustake/ et les exécutables
        }
        None => {
            println!("Démarrage de rustake");

            match config::load("Rustakefile.toml") {
                Ok(conf) => {
                    println!("{:?}", conf);

                    // On parcourt les cibles
                    for (name, target) in &conf.targets {
                        println!("Target '{}' : '{}'", name, target.command);
                        
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
