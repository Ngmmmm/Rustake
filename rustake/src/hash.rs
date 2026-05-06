use std::fs;
use std::path::Path;
use std::io;

/// Calcule l'empreinte blake3 d'un fichier.
/// 
/// Retourne une erreur `io::Error` si le fichier n'existe pas ou s'il y a un problème de lecture.
/// C'est un comportement attendu : si une cible n'a pas encore été compilée, le fichier n'existe pas donc recompilation
pub fn hash_file(path: &Path) -> io::Result<String> {
    //lit tout le contenu du fichier en mémoire en tant que bytes (octets)
    let content = fs::read(path)?;

    // Calcule le hash du contenu
    let hash = blake3::hash(&content);

    // Convertit le résultat binaire en une chaîne de caractères (hexadécimal)
    Ok(hash.to_hex().to_string())
}