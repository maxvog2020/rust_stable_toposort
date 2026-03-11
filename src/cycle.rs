use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleError<N> {
    pub cycle: Vec<N>,
}

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
