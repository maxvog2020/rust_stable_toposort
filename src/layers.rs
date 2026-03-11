use std::collections::{HashMap, HashSet};

use crate::{find_cycle, CycleError};

pub fn toposort_layers<N>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
) -> Result<Vec<Vec<N>>, CycleError<N>>
where
    N: Eq + std::hash::Hash + Clone,
{
    let nodes: Vec<N> = nodes.into_iter().collect();
    let edges: Vec<(N, N)> = edges.into_iter().collect();
    toposort_layers_impl(&nodes, edges, |i| i)
}

pub fn toposort_layers_by_key<N, K>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
    key: impl Fn(&N) -> K,
) -> Result<Vec<Vec<N>>, CycleError<N>>
where
    N: Eq + std::hash::Hash + Clone,
    K: Ord,
{
    let nodes: Vec<N> = nodes.into_iter().collect();
    let edges: Vec<(N, N)> = edges.into_iter().collect();
    toposort_layers_impl(&nodes, edges, |i| key(&nodes[i]))
}

fn toposort_layers_impl<N, K>(
    nodes: &[N],
    edges: impl IntoIterator<Item = (N, N)>,
    key: impl Fn(usize) -> K,
) -> Result<Vec<Vec<N>>, CycleError<N>>
where
    N: Eq + std::hash::Hash + Clone,
    K: Ord,
{
    let index_of: HashMap<N, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect();

    let mut in_degree: HashMap<N, u32> = HashMap::new();
    for n in nodes {
        in_degree.entry(n.clone()).or_insert(0);
    }
    let mut successors: HashMap<N, Vec<N>> = HashMap::new();
    for n in nodes {
        successors.entry(n.clone()).or_default();
    }

    for (a, b) in edges {
        if index_of.contains_key(&a) && index_of.contains_key(&b) {
            *in_degree.get_mut(&b).unwrap() += 1;
            successors.get_mut(&a).unwrap().push(b);
        }
    }

    let mut layers = Vec::new();
    let mut in_degree = in_degree;

    loop {
        let mut ready: Vec<(K, N)> = in_degree
            .iter()
            .filter(|&(_, &d)| d == 0)
            .map(|(n, _)| (key(index_of[n]), n.clone()))
            .collect();
        if ready.is_empty() {
            break;
        }
        ready.sort_by(|a, b| a.0.cmp(&b.0));
        let layer: Vec<N> = ready.into_iter().map(|(_, n)| n).collect();
        for n in &layer {
            in_degree.remove(n);
            for s in successors.get(n).into_iter().flat_map(|v| v.iter()) {
                if let Some(d) = in_degree.get_mut(s) {
                    *d -= 1;
                }
            }
        }
        layers.push(layer);
    }

    if layers.iter().map(|l| l.len()).sum::<usize>() != nodes.len() {
        let done: HashSet<N> = layers.iter().flat_map(|l| l.iter().cloned()).collect();
        let cycle = find_cycle(nodes, &successors, &done);
        return Err(CycleError { cycle });
    }
    Ok(layers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_ok() {
        let layers = toposort_layers::<&str>(Vec::new(), []).unwrap();
        assert!(layers.is_empty());
    }

    #[test]
    fn single_layer_one_node() {
        let layers = toposort_layers(["a"], []).unwrap();
        assert_eq!(layers, vec![vec!["a"]]);
    }

    #[test]
    fn cycle_err() {
        let r = toposort_layers(["a", "b"], [("a", "b"), ("b", "a")]);
        assert!(r.is_err());
    }

    #[test]
    fn by_key_orders_layer() {
        let nodes = ["B", "A", "C"];
        let edges = [("A", "C"), ("B", "C")];
        let layers = toposort_layers_by_key(nodes, edges, |n| *n).unwrap();
        assert_eq!(layers.len(), 2);
        assert_eq!(layers[0], ["A", "B"]);
        assert_eq!(layers[1], ["C"]);
    }
}
