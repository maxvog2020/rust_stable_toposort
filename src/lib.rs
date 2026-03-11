mod toposort;
mod layers;
mod scc;
mod condensation;

pub use toposort::{stable_toposort, stable_toposort_by_key};
pub use layers::{toposort_layers, toposort_layers_by_key};
pub use scc::{scc, scc_by_key};
pub use condensation::{Condensation, condensation, condensation_by_key, stable_toposort_scc, stable_toposort_scc_by_key};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleError<N> {
    pub cycle: Vec<N>,
}

pub(crate) fn find_cycle<N>(
    nodes: &[N],
    successors: &std::collections::HashMap<N, Vec<N>>,
    done: &std::collections::HashSet<N>,
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
    let mut path_pos: std::collections::HashMap<N, usize> =
        std::collections::HashMap::from([(start.clone(), 0)]);
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
