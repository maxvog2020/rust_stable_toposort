use rust_stable_toposort::{
    condensation, condensation_by_key, scc, scc_by_key, stable_toposort, stable_toposort_by_key,
    stable_toposort_scc, stable_toposort_scc_by_key, toposort_layers, toposort_layers_by_key,
};

#[test]
fn basic() {
    let nodes = ["A", "B", "C"];
    let edges = [("A", "C"), ("B", "C")];

    let order = stable_toposort(nodes, edges).unwrap();

    assert_eq!(order, vec!["A", "B", "C"]);
}

#[test]
fn stability() {
    let nodes = ["B", "A", "C"];
    let edges = [("A", "C"), ("B", "C")];

    let order = stable_toposort(nodes, edges).unwrap();

    assert_eq!(order, vec!["B", "A", "C"]);
}

#[test]
fn layers() {
    let nodes = ["A", "B", "C", "D"];
    let edges = [("A", "C"), ("B", "C"), ("C", "D")];

    let layers = toposort_layers(nodes, edges).unwrap();

    assert_eq!(layers, vec![vec!["A", "B"], vec!["C"], vec!["D"]]);
}

#[test]
fn diamond() {
    let nodes = ["A", "B", "C"];
    let edges = [("A", "C"), ("B", "C")];

    let order = stable_toposort(nodes, edges).unwrap();

    assert_eq!(order, vec!["A", "B", "C"]);
}

#[test]
fn independent_nodes() {
    let nodes = ["A", "B", "C"];
    let edges: [(&str, &str); 0] = [];

    let order = stable_toposort(nodes, edges).unwrap();

    assert_eq!(order, vec!["A", "B", "C"]);
}

#[test]
fn chain() {
    let nodes = ["A", "B", "C"];
    let edges = [("A", "B"), ("B", "C")];

    let order = stable_toposort(nodes, edges).unwrap();

    assert_eq!(order, vec!["A", "B", "C"]);
}

#[test]
fn cycle() {
    let nodes = ["A", "B", "C"];
    let edges = [("A", "B"), ("B", "C"), ("C", "A")];

    let result = stable_toposort(nodes, edges);

    let Err(e) = result else { panic!("expected cycle error") };
    assert!(!e.cycle.is_empty());
    let set: std::collections::HashSet<_> = e.cycle.iter().collect();
    assert!(set.contains(&"A") && set.contains(&"B") && set.contains(&"C"));
}

#[test]
fn layers_diamond() {
    let nodes = ["A", "B", "C", "D", "E"];
    let edges = [("A", "C"), ("B", "C"), ("C", "D"), ("C", "E")];

    let layers = toposort_layers(nodes, edges).unwrap();

    assert_eq!(layers.len(), 3);
    assert_eq!(layers[0], ["A", "B"]);
    assert_eq!(layers[1], ["C"]);
    assert_eq!(layers[2], ["D", "E"]);
}

#[test]
fn scc_single() {
    let nodes = ["A", "B", "C"];
    let edges: [(&str, &str); 0] = [];

    let components = scc(nodes, edges);

    assert_eq!(components.len(), 3);
    let mut flat: Vec<&str> = components.into_iter().flat_map(|c| c.into_iter()).collect();
    flat.sort();
    assert_eq!(flat, ["A", "B", "C"]);
}

#[test]
fn scc_cycle() {
    let nodes = ["A", "B", "C"];
    let edges = [("A", "B"), ("B", "C"), ("C", "A")];

    let components = scc(nodes, edges);

    assert_eq!(components.len(), 1);
    assert_eq!(components[0].len(), 3);
    assert!(components[0].contains(&"A"));
    assert!(components[0].contains(&"B"));
    assert!(components[0].contains(&"C"));
}

#[test]
fn condensation_dag() {
    let nodes = [1, 2, 3];
    let edges = [(1, 2), (2, 3)];

    let cond = condensation(nodes, edges);

    assert_eq!(cond.components.len(), 3);
    assert_eq!(cond.edges.len(), 2);
    let edge_set: std::collections::HashSet<_> = cond.edges.into_iter().collect();
    assert_eq!(edge_set.len(), 2);
}

#[test]
fn stable_toposort_scc_dag() {
    let nodes = ["A", "B", "C"];
    let edges = [("A", "B"), ("B", "C")];

    let sccs = stable_toposort_scc(nodes, edges);

    assert_eq!(sccs.len(), 3);
    for c in &sccs {
        assert_eq!(c.len(), 1, "DAG => each node is its own SCC");
    }
    let all: std::collections::HashSet<_> = sccs.iter().flat_map(|c| c.iter()).collect();
    assert_eq!(all.len(), 3);
    assert!(all.contains(&"A") && all.contains(&"B") && all.contains(&"C"));
}

#[test]
fn empty_graph() {
    let order = stable_toposort::<&str>(Vec::new(), []).unwrap();
    assert!(order.is_empty());
}

#[test]
fn single_node_no_edges() {
    let order = stable_toposort(["only"], []).unwrap();
    assert_eq!(order, ["only"]);
}

#[test]
fn layers_empty() {
    let layers = toposort_layers::<&str>(Vec::new(), []).unwrap();
    assert!(layers.is_empty());
}

#[test]
fn stable_toposort_by_key_custom_order() {
    let nodes = ["aaa", "b", "cc"];
    let edges = [("b", "cc"), ("aaa", "cc")];
    let order = stable_toposort_by_key(nodes, edges, |n| n.len()).unwrap();
    assert_eq!(order[0], "b");
    assert!(order.contains(&"aaa") && order.contains(&"cc"));
    let pos = |n: &str| order.iter().position(|&x| x == n).unwrap();
    assert!(pos("b") < pos("cc") && pos("aaa") < pos("cc"));
}

#[test]
fn layers_by_key_orders_within_layer() {
    let nodes = ["B", "A", "C", "D"];
    let edges = [("A", "C"), ("B", "C"), ("C", "D")];
    let layers = toposort_layers_by_key(nodes, edges, |n| *n).unwrap();
    assert_eq!(layers[0], ["A", "B"]);
    assert_eq!(layers[1], ["C"]);
    assert_eq!(layers[2], ["D"]);
}

#[test]
fn scc_by_key_deterministic_component_order() {
    let nodes = ["z", "y", "x"];
    let edges = [("x", "y"), ("y", "z"), ("z", "x")];
    let comps = scc_by_key(nodes, edges, |n| *n);
    assert_eq!(comps.len(), 1);
    assert_eq!(&comps[0], &["x", "y", "z"]);
}

#[test]
fn condensation_by_key_sorted_components() {
    let nodes = [3, 1, 2];
    let edges = [(1, 2), (2, 3), (3, 1)];
    let cond = condensation_by_key(nodes, edges, |&n| n);
    assert_eq!(cond.components.len(), 1);
    assert_eq!(&cond.components[0], &[1, 2, 3]);
}

#[test]
fn stable_toposort_scc_by_key_integration() {
    let nodes = ["C", "A", "B"];
    let edges = [("A", "B"), ("B", "C")];
    let sccs = stable_toposort_scc_by_key(nodes, edges, |n| *n);
    assert_eq!(sccs.len(), 3);
    assert!(sccs.iter().all(|c| c.len() == 1));
    let flat: std::collections::HashSet<_> = sccs.into_iter().flatten().collect();
    assert_eq!(flat.len(), 3);
    assert!(flat.contains("A") && flat.contains("B") && flat.contains("C"));
}

#[test]
fn edges_ignored_for_unknown_nodes() {
    let nodes = ["A", "B"];
    let edges = [("A", "B"), ("A", "Z"), ("Z", "B")];
    let order = stable_toposort(nodes, edges).unwrap();
    assert_eq!(order, ["A", "B"]);
}

#[test]
fn layers_cycle_error() {
    let r = toposort_layers(["a", "b", "c"], [("a", "b"), ("b", "c"), ("c", "a")]);
    assert!(r.is_err());
}
