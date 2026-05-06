use std::process::Command;
use std::fmt;
use std::process::ExitStatus;

#[derive(Debug)]
enum ExecError {
    EmptyCommand, // Erreur quand une commande est vide
    SpawnFailed(String, std::io::Error), // Erreur quand une commande ne peut pas être exécuté par status()
    ErrorExit(String, ExitStatus), // Erreur quand une commande échoue pendant son exécution
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            ExecError::EmptyCommand => write!(f, "La commande est vide"),
            ExecError::SpawnFailed(cmd, err) => write!(f, "Impossible de lancer {} : {}", cmd, err),
            ExecError::ErrorExit(cmd, status) => write!(f, "La commande {} a échoué : {}", cmd, status)
        }
    }
}

// On implémente le trait Error pour gérer les erreurs renvoyé par status pour gérer nos cas
// C'est possible car on a bien derive(Debug) et on implémente Display
impl std::error::Error for ExecError {}

/// Execute une commande sous la forme d'une chaîne de caractères
fn run(command: &str) -> Result<(), ExecError> {
    let cmd = command.trim(); // On enlève les espaces pour voir si c'est pas juste vide
    if cmd.is_empty() {
        return Err(ExecError::EmptyCommand);
    }

    let status = string_to_command(cmd)
        .status() // Execute et attends la fin pour renvoyer la valeur de retour de l'exécution
        // On gère les erreurs d'exécution.
        // La map_err converti la io::Error en ExecError pour traiter les erreurs comem on le veut
        .map_err(|err| ExecError::SpawnFailed(cmd.to_string(), err))?;  // ? pour dépiler la valeur de retour et continuer si possible

    // On regarde si l'exécution à fonctionné sinon erreur
    if status.success() {
        Ok(())
    } else {
        Err(ExecError::ErrorExit(cmd.to_string(), status))
    }
}

/// Permet d'exécuter les commandes d'un tableau de String
/// Les erreurs d'exécution sont gérées individuellement dans 'run'
pub fn run_all(commands: &[String]) -> Result<(), ExecError> {
    for cmd in commands {
        run(cmd)?; // ? pour dépiler la valeur de retour de run
    }
    Ok(())
}

/// Permet de créer l'objet Command en fonction de l'os
/// -c et /c permet à sh et cmd d'exécuter une commande qui est une string
/// Source: https://stackoverflow.com/questions/43292357/how-can-one-detect-the-os-type-using-rust
fn string_to_command(command: &str) -> Command {
    if cfg!(unix) {
        // On décompose bien la création contrairement aux exemples de la documentation car
        // on veut renvoyer un type Command et pas &mut Command
        let mut c = Command::new("sh");
        c.arg("-c").arg(command);
        c
    } else {
        let mut c = Command::new("cmd");
        c.args(["/C", command]);
        c
    }
}
