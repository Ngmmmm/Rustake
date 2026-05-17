use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io;

/// Représente le contenu de notre fichier de cache
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cache {
    /// Associe un chemin de fichier à son hash blake3
    pub hashes: HashMap<String, String>,
}

impl Cache {
    /// Tente de charger le cache depuis le disque
    /// S'il existe pas retourne un cache vide
    pub fn load() -> Self {
        let cache_path = Path::new(".rustake/cache");

        if cache_path.exists() {
            if let Ok(content) = fs::read_to_string(cache_path) {
                // On tente de désérialiser le JSON Si ça échoue, on repart de zéro
                if let Ok(cache) = serde_json::from_str(&content) {
                    return cache;
                }
            }
        }

        // Si aucun cache n'existe ou s'il est corrompu on renvoie un cache vide
        Cache::default()
    }

    /// Sauvegarde le cache actuel sur le disque
    pub fn save(&self) -> io::Result<()> {
        let dir_path = Path::new(".rustake");

        //On s'assure que le dossier .rustake existe
        if !dir_path.exists() {
            fs::create_dir_all(dir_path)?;
        }

        let cache_path = dir_path.join("cache");

        //On sérialise notre structure en JSON
        let json_content = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        //On écrit dans le fichier
        fs::write(cache_path, json_content)?;

        Ok(())
    }

    /// Met à jour le hash d'un fichier spécifique dans le cache en mémoire
    pub fn update_hash(&mut self, file_path: &str, new_hash: String) {
        self.hashes.insert(file_path.to_string(), new_hash);
    }

    /// Vérifie si un fichier a changé en comparant son hash actuel avec celui en cache
    /// Retourne true si le fichier a changé ou s'il n'était pas dans le cache
    pub fn has_changed(&self, file_path: &str, current_hash: &str) -> bool {
        match self.hashes.get(file_path) {
            Some(cached_hash) => cached_hash != current_hash,
            None => true, // Le fichier n'est pas dans le cache donc on considère qu'il a changé
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknow_file_is_changed() {
        let cache = Cache::default();
        assert!(cache.has_changed("src/main.rs", "abc123")); // pas dans l'hashmap donc considéré comme changé
    }

    #[test]
    fn test_has_changed_same_hash() {
        let mut cache = Cache::default();
        cache.update_hash("src/main.rs", "abc123".to_string());
        assert!(!cache.has_changed("src/main.rs", "abc123"));
    }

    #[test]
    fn test_has_changed_different_hash() {
        let mut cache = Cache::default();
        cache.update_hash("src/main.rs", "abc123".to_string());
        assert!(cache.has_changed("src/main.rs", "xyz999"));
    }
}
