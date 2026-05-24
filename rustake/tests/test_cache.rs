use rustake::cache::Cache;
use std::fs;
use std::path::Path;
use std::io;

fn clean_test_file() -> io::Result<()> {
    let dir = Path::new(".rustake");
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    Ok(())
}

#[test]
fn integration_cache_save_load() {
    clean_test_file().unwrap(); // vide si un .rustake existe déjà
    let mut cache = Cache::load();
    assert!(cache.has_changed("src/main.rs", "hash_test"));
    cache.update_hash("src/main.rs", "hash_test".to_string());
    cache.save().unwrap();

    // charge après sauvegarde
    let mut cache2 = Cache::load();
    assert!(!cache.has_changed("src/main.rs", "hash_test"));
    assert!(cache.has_changed("src/main.rs", "new_hash"));
    cache2.update_hash("src/main.rs", "new_hash".to_string()); // nouveau hash
    cache2.save().unwrap();

    let mut cache3 = Cache::load();
    assert!(!cache3.has_changed("src/main.rs", "new_hash")); // test nouveau hash

    clean_test_file().unwrap();
}
