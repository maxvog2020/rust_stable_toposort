//! Cycle detection for directed graphs.

use std::collections::{HashMap, HashSet};

/// Error returned when a topological sort is attempted on a graph that contains a cycle.
///
/// The `cycle` field holds a sequence of nodes that form a cycle: the first and last
/// element are the same, and each consecutive pair is an edge in the graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleError<N> {
    /// A sequence of nodes forming a cycle (first node repeated at the end).
    pub cycle: Vec<N>,
}

/// Finds a cycle in the subgraph induced by nodes not in `done`.
///
/// Uses DFS to detect a back edge. Returns a non-empty cycle (first and last node
/// equal) if one exists, otherwise an empty vector. Only considers nodes in `nodes`
/// and edges present in `successors`; nodes in `done` are treated as removed.
///
/// # Arguments
///
/// * `nodes` - All nodes of the graph (order can affect which cycle is found).
/// * `successors` - Adjacency list: for each node, the list of nodes it has an edge to.
/// * `done` - Nodes to exclude from the search (e.g. already placed in the sort).
pub fn find_cycle<N>(
    nodes: &[N],
    successors: &HashMap<N, Vec<N>>,
    done: &HashSet<N>,
) -> Vec<N>
where
    N: Eq + std::hash::Hash + Clone,
{
    let remaining: Vec<N> = nodes.iter().cloned().filter(|n| !done.contains(n)).collect();
    if remaining.is_empty() {
        return vec![];
    }
    let start = remaining[0].clone();
    let mut path = vec![start.clone()];
    let mut path_pos: HashMap<N, usize> = HashMap::from([(start.clone(), 0)]);
    let mut work: Vec<(N, usize)> = vec![(start, 0)];
    while let Some((cur, i)) = work.pop() {
        let succs = successors.get(&cur).map(|v| v.as_slice()).unwrap_or(&[]);
        if i < succs.len() {
            let s = succs[i].clone();
            work.push((cur.clone(), i + 1));
            if !done.contains(&s) {
                if let Some(&pos) = path_pos.get(&s) {
                    let mut cycle: Vec<N> = path[pos..].to_vec();
                    cycle.push(s);
                    return cycle;
                }
                path_pos.insert(s.clone(), path.len());
                path.push(s.clone());
                work.push((s, 0));
            }
            continue;
        }
        path.pop();
        path_pos.remove(&cur);
    }
    vec![]
}
