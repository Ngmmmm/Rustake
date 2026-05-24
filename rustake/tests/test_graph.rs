use rustake::graph::DepGraph;
use std::collections::HashMap;

/// Construit un graphe autrement qu'avec une config (pour le test)
fn make_graph(edges: &[(&str, &[&str])]) -> DepGraph {
    let mut dependencies = HashMap::new();
    for (node, deps) in edges {
        dependencies
            .entry(node.to_string())
            .or_insert_with(Vec::new)
            .extend(deps.iter().map(|s| s.to_string()));
        for dep in *deps {
            dependencies.entry(dep.to_string()).or_default();
        }
    }
    DepGraph { dependencies }
}

// graphe: c -> b -> a
// attendu: a, b, c
#[test]
fn integration_topo_sort() {
    let g = make_graph(&[("c", &["b"]), ("b", &["a"]), ("a", &[])]);
    let sorted = g.topogical_sort().unwrap();

    let idx = |name: &str| sorted.iter().position(|x| x == name).unwrap();
    assert!(idx("a") < idx("b"));
    assert!(idx("b") < idx("c"));
}
