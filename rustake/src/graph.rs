use std::collections::HashMap;
use crate::config::Config;

#[derive(Debug)]
pub enum GraphError {
    HasCycleError,
}

pub struct DepGraph {
    // String = Noeud, Vec<String> = fils du noeud
    pub dependencies: HashMap<String, Vec<String>>,
}

impl DepGraph {
    /// Fonction qui va construire un graphe depuis une configuration donnée
    /// Un noeud du graphe est une dependance ou un nom de target
    pub fn from_config(config: &Config) -> Self {
	    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
	    for (name, config) in &config.targets {
	        // On regarde si un noeud 'name' existe, or_default() fait un Vec<String> vide si le noeud est pas la
	        // On rempli le Vec<String> avec extend
	        deps
		        .entry(name.clone())
		        .or_default()
		        .extend(config.depends.iter().cloned());
	        // On vérifie que les noeuds des dépendances existent
	        for dep in &config.depends {
		        deps.entry(dep.clone()).or_default();
	        }
	    }
	    DepGraph { dependencies: deps }
    }

    ///On regarde s'il y a des cycles depuis un noeud à l'indice 'POS' et ses voisins.
    ///PARAMETERS: POS est l'index du noeud depuis lequel on veut détecter un cycle, NODE_ADJ est la liste des index des noeuds voisins
    ///a celui à l'index POS, VISITED est une pile des noeuds déjà visités, RECURSION est une pile des noeuds du parcours courant.
    ///RETURN VALUE: TRUE si il y a un cycle, FALSE sinon
    fn cycle_detection(pos: usize, node_adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, recursion: &mut Vec<bool>) -> bool{
	    if recursion[pos] {
	        return true; // Si on repasse sur un noeud du parcours courant alors on a fait une boucle
	    }
	    if visited[pos] {
	        return false; // Si déjà visité on passe
	    }

	    visited[pos] = true;
	    recursion[pos] = true;

	    // On regarde s'il y a un cycle pour tous les noeuds adjacents
	    for &i in &node_adj[pos] {
	        if Self::cycle_detection(i, node_adj, visited, recursion) {
		        return true;
	        }
	    }
	    recursion[pos] = false;
	    false
    }

    ///Permet de détecter si un graphe de dépendance possède un ou des cycles
    ///Pour cela, on utilise l'algorithme DFS pour parcourir tout le graphe.
    ///RETURN VALUE: TRUE si il y a un cycle, FALSE sinon
    pub fn dfs_cycle_detection(&self) -> bool {
	    let nodes: Vec<&String> = self.dependencies.keys().collect();
	    let n = nodes.len();
	    // On construit un vecteur qui pour chaque index, a un vecteur qui contient la liste des index des voisins
	    // On parcourt chaque dépendance, retrouve sa position et on recréer le vecteur avec les index
	    let adjacent: Vec<Vec<usize>> = nodes.iter().map(|node| {
	        self.dependencies[*node]
		        .iter()
		        .filter_map(|dep| nodes.iter().position(|&n| n == dep))
		        .collect()
	    }).collect();

	    let mut visited = vec![false; n];
	    let mut recursion = vec![false; n];

	    // On applique la méthode pour la détection de cycle sur tous les noeuds si le graphe n'est pas connexe
	    for i in 0..n{
	        if !visited[i] && Self::cycle_detection(i, &adjacent, &mut visited, &mut recursion) {
		        return true;
	        }
	    }
	    false
    }

    ///On suit le même parcours DFS que pour l'algorithme de détection des cycles mais,
    ///Quand un noeud n'est pas visité on l'ajoute à notre pile pour construire le tri
    ///Comme dans ce projet on l'utilise obligatoirement après une détection de cycle qui n'a rien trouvé,
    ///on a la certitude que le tri n'est pas mal géré à cause d'un cycle
    fn topo_dfs(pos: usize, node_adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, stack: &mut Vec<usize>) {
	    if visited[pos] { return; } // Déjà visité

	    visited[pos] = true;

	    for &i in &node_adj[pos] {
            Self::topo_dfs(i, node_adj, visited, stack);
	    }
	    stack.push(pos);
    }

    ///
    pub fn topogical_sort(&self) -> Result<Vec<String>, GraphError> {
	    if self.dfs_cycle_detection() {
	        return Err(GraphError::HasCycleError);
	    }

	    let nodes: Vec<&String> = self.dependencies.keys().collect();
	    let n = nodes.len();

	    // On construit un vecteur qui pour chaque index, a un vecteur qui contient la liste des index des voisins
	    // On parcourt chaque dépendance, retrouve sa position et on recréer le vecteur avec les index
	    let adjacent: Vec<Vec<usize>> = nodes.iter().map(|node| {
	        self.dependencies[*node]
		        .iter()
		        .filter_map(|dep| nodes.iter().position(|&n| n == dep))
		        .collect()
	    }).collect();

	    let mut visited = vec![false; n];
	    let mut stack: Vec<usize> = Vec::new();

	    for i in 0..n{
	        if !visited[i] {
		        Self::topo_dfs(i, &adjacent, &mut visited, &mut stack);
	        }
	    }

	    // Renvoi le tri
	    Ok(stack.iter().map(|&i| nodes[i].clone()).collect())
    }
}
