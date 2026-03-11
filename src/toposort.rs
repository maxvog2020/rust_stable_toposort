//! Stable topological sort using Kahn's algorithm.

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

use crate::CycleError;

/// Stable topological sort: returns a valid topological order that minimizes
/// inversions relative to the order in `nodes`. `(a, b)` in `edges` means a → b.
pub fn stable_toposort<N>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
) -> Result<Vec<N>, CycleError>
where
    N: Eq + std::hash::Hash + Clone,
{
    let nodes: Vec<N> = nodes.into_iter().collect();
    let edges: Vec<(N, N)> = edges.into_iter().collect();
    stable_toposort_impl(&nodes, edges, |i| i)
}

/// Like `stable_toposort`, but stability is determined by `key(n)` instead of node identity.
pub fn stable_toposort_by_key<N, K>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
    key: impl Fn(&N) -> K,
) -> Result<Vec<N>, CycleError>
where
    N: Eq + std::hash::Hash + Clone,
    K: Ord,
{
    let nodes: Vec<N> = nodes.into_iter().collect();
    let edges: Vec<(N, N)> = edges.into_iter().collect();
    stable_toposort_impl(&nodes, edges, |i| key(&nodes[i]))
}

/// Common implementation: `key` maps node index → ordering key.
fn stable_toposort_impl<N, K>(
    nodes: &[N],
    edges: impl IntoIterator<Item = (N, N)>,
    key: impl Fn(usize) -> K,
) -> Result<Vec<N>, CycleError>
where
    N: Eq + std::hash::Hash + Clone,
    K: Ord,
{
    let mut index_of: HashMap<N, usize> = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        index_of.insert(n.clone(), i);
    }

    let mut in_degree: HashMap<N, u32> = HashMap::new();
    for n in nodes {
        in_degree.entry(n.clone()).or_insert(0);
    }
    let mut successors: HashMap<N, Vec<N>> = HashMap::new();
    for n in nodes {
        successors.entry(n.clone()).or_default();
    }

    for (a, b) in edges {
        if !index_of.contains_key(&a) || !index_of.contains_key(&b) {
            continue;
        }
        *in_degree.get_mut(&b).unwrap() += 1;
        successors.get_mut(&a).unwrap().push(b);
    }

    let mut ready: BinaryHeap<Reverse<(K, usize)>> = BinaryHeap::new();
    for (i, n) in nodes.iter().enumerate() {
        if in_degree.get(n).copied().unwrap_or(1) == 0 {
            ready.push(Reverse((key(i), i)));
        }
    }

    let mut result = Vec::with_capacity(nodes.len());
    while let Some(Reverse((_, i))) = ready.pop() {
        let n = nodes[i].clone();
        result.push(n.clone());
        for s in successors.get(&n).into_iter().flat_map(|v| v.iter()) {
            let s = s.clone();
            let d = in_degree.get_mut(&s).unwrap();
            *d -= 1;
            if *d == 0 {
                let j = index_of[&s];
                ready.push(Reverse((key(j), j)));
            }
        }
    }

    if result.len() != nodes.len() {
        return Err(CycleError);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_nodes() {
        let order = stable_toposort::<&str>(Vec::<&str>::new(), []).unwrap();
        assert!(order.is_empty());
    }

    #[test]
    fn single_node() {
        let order = stable_toposort(["x"], []).unwrap();
        assert_eq!(order, ["x"]);
    }

    #[test]
    fn by_key_orders_by_key() {
        let nodes = ["B", "A", "C"];
        let edges = [("A", "C"), ("B", "C")];
        // key = identity: order within same "level" follows key order A < B < C
        let order = stable_toposort_by_key(nodes, edges, |n| *n).unwrap();
        assert_eq!(order, ["A", "B", "C"]);
    }

    #[test]
    fn cycle_returns_err() {
        let r = stable_toposort(["a", "b"], [("a", "b"), ("b", "a")]);
        assert!(r.is_err());
    }
}
