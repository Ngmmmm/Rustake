# Mon outil de build en rust

# Rustake

**Rustake** est un outil d'automatisation de compilation inspiré de `make`en Rust.

## But du projet
Le but de Rustake est de fournir un système de build simple, rapide et efficace. Il lit un fichier de configuration (`Rustakefile.toml`) et exécute les commandes nécessaires pour construire un projet de manière optimisée. Son objectif principal est de faire gagner du temps lors du développement en s'assurant que seuls les fichiers modifiés ou impactés sont recompilés.

## Fonctionnalités
* **Initialisation rapide :** La commande `init` permet de générer automatiquement un fichier `Rustakefile.toml` de base pour démarrer un nouveau projet.
* **Build incrémental :** Rustake vérifie isi les fichiers sources ont été modifiés avant d'exécuter une commande de compilation.
* **Nettoyage :** La commande `clean` supprime le cache de compilation interne ainsi que les fichiers cibles générés.
* **Détection de dépendances circulaires :** Le programme analyse les dépendances et s'arrête avec une erreur fatale s'il détecte un cycle, évitant ainsi les boucles infinies.
* **Compatibilité Multi-plateforme :** Les commandes de build sont exécutées nativement via `sh -c` sur les systèmes Unix et `cmd /C` sur les systèmes Windows.

## Particularités techniques
* **Graphe de dépendances :** Les dépendances entre les cibles et les fichiers sont modélisées sous la forme d'un graphe orienté stocké en mémoire.
* **Algorithme DFS & Tri topologique :** Le graphe est parcouru en profondeur (Depth-First Search) pour la détection de cycles. Si aucun cycle n'est trouvé, un tri topologique est généré pour garantir que les dépendances sont compilées dans le bon ordre.
* **Hachage performant avec Blake3 :** Pour vérifier si un fichier a changé, Rustake calcule l'empreinte cryptographique de son contenu avec l'algorithme `blake3`.
* **Système de cache persistant :** Les empreintes des fichiers sont sauvegardées et chargées depuis un fichier JSON situé dans un dossier caché `.rustake/cache`. La sérialisation est gérée par `serde`.
* **Interface CLI robuste :** L'interface en ligne de commande est construite à l'aide de la bibliothèque `clap`, offrant un parsing propre des sous-commandes.
* **Configuration TOML :** Le fichier de règles est défini au format TOML et chargé dynamiquement en mémoire avec la crate `toml`.

## Utilisation

### Compilation du projet (Rustake)
Assurez-vous de posséder la chaîne d'outils Rust (et `cargo`), puis compilez le projet :
`cargo build --release`

L'exécutable généré sera disponible dans le dossier `target/release/`.

### Commandes disponibles
Une fois Rustake compilé , vous pouvez utiliser les commandes suivantes :

* **Initialiser un projet :**
  Génère un fichier de configuration standard `Rustakefile.toml`.
  `rustake init`

* **Lancer la compilation :**
  Construit le graphe, vérifie le cache, affiche les fichiers à jour et compile ceux qui en ont besoin.
  `rustake`

* **Nettoyer le projet :**
  Supprime le dossier interne de cache `.rustake` ainsi que toutes les cibles définies dans la configuration.
  `rustake clean`

### Format du `Rustakefile.toml`
Voici la structure attendue pour votre fichier de configuration :

```toml
[targets.program]
depends = ["src/main.rs"]
command = "rustc src/main.rs -o program"
