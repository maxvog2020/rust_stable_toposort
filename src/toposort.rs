//! Stable topological sort for directed acyclic graphs (DAGs).

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;

use crate::cycle::{find_cycle, CycleError};

/// Computes a deterministic topological order of `nodes` respecting `edges`.
///
/// The order is stable: given the same `nodes` and `edges`, the result is always
/// the same. Nodes with no ordering constraint are ordered by their position in
/// the `nodes` iterator.
///
/// # Errors
///
/// Returns [`crate::cycle::CycleError`] if the graph contains a cycle. The error's `cycle` field
/// contains a sequence of nodes that form a cycle.
///
/// # Examples
///
/// ```rust
/// use stable_toposort::toposort::toposort;
///
/// let nodes = ["prepare", "compile", "link"];
/// let edges = [("prepare", "compile"), ("compile", "link")];
/// let order = toposort(nodes, edges).unwrap();
/// assert_eq!(order, ["prepare", "compile", "link"]);
/// ```
pub fn toposort<N>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
) -> Result<Vec<N>, CycleError<N>>
where
    N: Eq + std::hash::Hash + Clone,
{
    let nodes: Vec<N> = nodes.into_iter().collect();
    let edges: Vec<(N, N)> = edges.into_iter().collect();
    toposort_impl(&nodes, edges, |i| i)
}

/// Computes a deterministic topological order, ordering ties by `key`.
///
/// Same as [`toposort`], but when multiple nodes are valid as the next in
/// the order, they are ordered by comparing `key(node)`. This gives full control
/// over the resulting order (e.g. alphabetical, or by priority).
///
/// # Errors
///
/// Returns [`crate::cycle::CycleError`] if the graph contains a cycle.
///
/// # Examples
///
/// ```rust
/// use stable_toposort::toposort::toposort_by_key;
///
/// let nodes = ["B", "A", "C"];
/// let edges = [("A", "C"), ("B", "C")];
/// let order = toposort_by_key(nodes, edges, |n| *n).unwrap();
/// assert_eq!(order, ["A", "B", "C"]);
/// ```
pub fn toposort_by_key<N, K>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
    key: impl Fn(&N) -> K,
) -> Result<Vec<N>, CycleError<N>>
where
    N: Eq + std::hash::Hash + Clone,
    K: Ord,
{
    let nodes: Vec<N> = nodes.into_iter().collect();
    let edges: Vec<(N, N)> = edges.into_iter().collect();
    toposort_impl(&nodes, edges, |i| key(&nodes[i]))
}

/// Computes a deterministic topological order of indices `0..size` from edge list.
///
/// The graph has nodes `0, 1, ..., size - 1` and edges given as pairs of indices.
/// Indices in `edges` must be in range; out-of-range pairs are ignored.
/// The order is stable: ties between ready nodes are broken by the index (smaller
/// index first). Equivalent to [`toposort`](toposort)(0..size, edges) with nodes
/// as indices.
///
/// # Errors
///
/// Returns [`crate::cycle::CycleError`] if the graph contains a cycle. The error's
/// `cycle` field contains indices that form a cycle.
///
/// # Examples
///
/// ```rust
/// use stable_toposort::toposort::toposort_indices;
///
/// let edges = [(0, 1), (1, 2)];
/// let order = toposort_indices(edges, 3).unwrap();
/// assert_eq!(order, [0, 1, 2]);
/// ```
pub fn toposort_indices(
    edges: impl IntoIterator<Item = (usize, usize)>,
    size: usize,
) -> Result<Vec<usize>, CycleError<usize>>
{
    let indices = 0..size;
    toposort(indices, edges)
}

/// Computes a deterministic topological order of indices using a key slice for tie-breaking.
///
/// The graph has nodes `0, 1, ..., keys.len() - 1` and edges given as pairs of indices.
/// Indices in `edges` must be in range; out-of-range pairs are ignored.
/// When multiple nodes are valid as the next in the order, they are ordered by `keys[i]`.
/// This allows ordering by external data (e.g. names, priorities) without building
/// node/edge collections of that type.
///
/// # Errors
///
/// Returns [`crate::cycle::CycleError`] if the graph contains a cycle.
///
/// # Examples
///
/// ```rust
/// use stable_toposort::toposort::toposort_indices_with_keys;
///
/// let keys = ["B", "A", "C"];
/// let edges = [(1, 2), (0, 2)];
/// let order = toposort_indices_with_keys(edges, &keys).unwrap();
/// assert_eq!(order, [1, 0, 2]);
/// ```
pub fn toposort_indices_with_keys<K>(
    edges: impl IntoIterator<Item = (usize, usize)>,
    keys: &[K],
) -> Result<Vec<usize>, CycleError<usize>>
where
    K: Ord,
{
    let indices = 0..keys.len();
    toposort_by_key(indices, edges, |&i| &keys[i])
}

fn toposort_impl<N, K>(
    nodes: &[N],
    edges: impl IntoIterator<Item = (N, N)>,
    key: impl Fn(usize) -> K,
) -> Result<Vec<N>, CycleError<N>>
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
        let done: HashSet<N> = result.iter().cloned().collect();
        let cycle = find_cycle(nodes, &successors, &done);
        return Err(CycleError { cycle });
    }
    Ok(result)
}
