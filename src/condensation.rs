use std::collections::HashSet;

use crate::scc;
use crate::stable_toposort;

#[derive(Debug, Clone)]
pub struct Condensation<N> {
    pub components: Vec<Vec<N>>,
    pub edges: Vec<(usize, usize)>,
}

pub fn condensation<N>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
) -> Condensation<N>
where
    N: Eq + std::hash::Hash + Clone,
{
    let nodes: Vec<N> = nodes.into_iter().collect();
    let edges: Vec<(N, N)> = edges.into_iter().collect();

    let components = scc(nodes.clone(), edges.iter().cloned());
    let comp_of: std::collections::HashMap<N, usize> = components
        .iter()
        .enumerate()
        .flat_map(|(i, c)| c.iter().map(move |n| (n.clone(), i)))
        .collect();

    let mut edge_set: HashSet<(usize, usize)> = HashSet::new();
    for (a, b) in &edges {
        if let (Some(&i), Some(&j)) = (comp_of.get(a), comp_of.get(b)) {
            if i != j {
                edge_set.insert((i, j));
            }
        }
    }
    let edges = edge_set.into_iter().collect();

    Condensation { components, edges }
}

pub fn condensation_by_key<N, K>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
    key: impl Fn(&N) -> K,
) -> Condensation<N>
where
    N: Eq + std::hash::Hash + Clone,
    K: Ord,
{
    let mut cond = condensation(nodes, edges);
    for comp in &mut cond.components {
        comp.sort_by_key(&key);
    }
    cond
}

pub fn stable_toposort_scc<N>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
) -> Vec<Vec<N>>
where
    N: Eq + std::hash::Hash + Clone,
{
    let cond = condensation(nodes, edges);
    let order = stable_toposort(
        0..cond.components.len(),
        cond.edges.iter().copied(),
    )
    .expect("condensation is a DAG");
    order
        .into_iter()
        .map(|i| cond.components[i].clone())
        .collect()
}

pub fn stable_toposort_scc_by_key<N, K>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
    key: impl Fn(&N) -> K,
) -> Vec<Vec<N>>
where
    N: Eq + std::hash::Hash + Clone,
    K: Ord,
{
    let mut sccs = stable_toposort_scc(nodes, edges);
    for comp in &mut sccs {
        comp.sort_by_key(&key);
    }
    sccs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dag_components_singleton() {
        let cond = condensation([1, 2], [(1, 2)]);
        assert_eq!(cond.components.len(), 2);
        assert_eq!(cond.edges.len(), 1);
    }

    #[test]
    fn by_key_sorts_components() {
        let nodes = ["B", "A", "C"];
        let edges = [("A", "B"), ("B", "C"), ("C", "A")];
        let cond = condensation_by_key(nodes, edges, |n| *n);
        assert_eq!(cond.components.len(), 1);
        assert_eq!(&cond.components[0], &["A", "B", "C"]);
    }

    #[test]
    fn stable_toposort_scc_by_key_orders_within_components() {
        let nodes: [&str; 3] = ["C", "A", "B"];
        let edges = [("A", "B"), ("B", "C")];
        let sccs = super::stable_toposort_scc_by_key(nodes, edges, |n: &&str| *n);
        assert_eq!(sccs.len(), 3);
        assert_eq!(sccs[0], ["A"]);
        assert_eq!(sccs[1], ["B"]);
        assert_eq!(sccs[2], ["C"]);
    }
}
