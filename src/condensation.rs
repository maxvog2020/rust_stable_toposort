//! Condensation (DAG of SCCs) and topological sort of strongly connected components.

use std::collections::HashSet;

use crate::scc::scc;
use crate::toposort::toposort;

/// The condensation of a directed graph: one node per SCC, edges between components.
///
/// The condensation is always a DAG. Node indices refer to `components`: edge `(i, j)`
/// means there is at least one edge from some node in `components[i]` to some node in
/// `components[j]`.
#[derive(Debug, Clone)]
pub struct Condensation<N> {
    /// The strongly connected components; index `i` corresponds to component `i`.
    pub components: Vec<Vec<N>>,
    /// Edges between components as pairs of component indices `(from, to)`.
    pub edges: Vec<(usize, usize)>,
}

/// Builds the condensation of the graph (DAG of strongly connected components).
///
/// Each element of `components` is a strongly connected component. The `edges` list
/// contains pairs of component indices: `(i, j)` means there exists an edge from some
/// node in `components[i]` to some node in `components[j]`. The order of components
/// and of nodes within a component is unspecified; use [`condensation_by_key`] for
/// a deterministic order.
///
/// # Examples
///
/// ```rust
/// use stable_toposort::condensation::condensation;
///
/// let nodes = [1, 2, 3];
/// let edges = [(1, 2), (2, 3)];
/// let cond = condensation(nodes, edges);
/// assert_eq!(cond.components.len(), 3);
/// assert_eq!(cond.edges.len(), 2);
/// ```
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

/// Builds the condensation with components and nodes within each component ordered by `key`.
///
/// Same as [`condensation`], but the order of components and the order of nodes within
/// each component are determined by sorting with `key`.
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

/// Returns strongly connected components in topological order.
///
/// Computes the condensation (DAG of SCCs) and then topologically sorts it. The result
/// is a vector of components (each a `Vec<N>`), in an order such that all edges between
/// components go from an earlier component to a later one. The order of nodes within
/// each component is unspecified; use [`toposort_scc_by_key`] to fix it.
///
/// The graph's condensation is always a DAG, so this never returns an error.
///
/// # Examples
///
/// ```rust
/// use stable_toposort::condensation::toposort_scc;
///
/// let nodes = ["a", "b", "c"];
/// let edges = [("a", "b"), ("b", "c")];
/// let sccs = toposort_scc(nodes, edges);
/// assert_eq!(sccs.len(), 3);
/// ```
pub fn toposort_scc<N>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
) -> Vec<Vec<N>>
where
    N: Eq + std::hash::Hash + Clone,
{
    let cond = condensation(nodes, edges);
    let order = toposort(
        0..cond.components.len(),
        cond.edges.iter().copied(),
    )
    .expect("condensation is a DAG");
    order
        .into_iter()
        .map(|i| cond.components[i].clone())
        .collect()
}

/// Returns strongly connected components in topological order, with nodes ordered by `key`.
///
/// Same as [`toposort_scc`], but nodes within each component are sorted by
/// `key`, giving a fully deterministic result.
///
/// # Examples
///
/// ```rust
/// use stable_toposort::condensation::toposort_scc_by_key;
///
/// let nodes = ["C", "A", "B"];
/// let edges = [("A", "B"), ("B", "C")];
/// let sccs = toposort_scc_by_key(nodes, edges, |n| *n);
/// assert_eq!(sccs, vec![vec!["A"], vec!["B"], vec!["C"]]);
/// ```
pub fn toposort_scc_by_key<N, K>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
    key: impl Fn(&N) -> K,
) -> Vec<Vec<N>>
where
    N: Eq + std::hash::Hash + Clone,
    K: Ord,
{
    let mut sccs = toposort_scc(nodes, edges);
    for comp in &mut sccs {
        comp.sort_by_key(&key);
    }
    sccs
}
