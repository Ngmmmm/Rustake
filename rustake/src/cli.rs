use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rustake")]
#[command(about = "Equivalent make en rust", long_about = None)]
pub struct Cli {
    /// Commande spécifique à exécuter (si vide, lance le build par défaut)
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// sous commandes dispo
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Crée un Rustakefile.toml par défaut
    Init,
    /// Supprime les cibles et le cache
    Clean,
}