use std::collections::HashMap;

pub fn scc<N>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
) -> Vec<Vec<N>>
where
    N: Eq + std::hash::Hash + Clone,
{
    scc_impl(nodes, edges)
}

pub fn scc_by_key<N, K>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
    key: impl Fn(&N) -> K,
) -> Vec<Vec<N>>
where
    N: Eq + std::hash::Hash + Clone,
    K: Ord,
{
    let mut components = scc_impl(nodes, edges);
    for comp in &mut components {
        comp.sort_by_key(&key);
    }
    components
}

fn scc_impl<N>(
    nodes: impl IntoIterator<Item = N>,
    edges: impl IntoIterator<Item = (N, N)>,
) -> Vec<Vec<N>>
where
    N: Eq + std::hash::Hash + Clone,
{
    let nodes: Vec<N> = nodes.into_iter().collect();
    let n = nodes.len();
    let to_idx: HashMap<N, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, x)| (x.clone(), i))
        .collect();

    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (a, b) in edges {
        if let (Some(&i), Some(&j)) = (to_idx.get(&a), to_idx.get(&b)) {
            adj[i].push(j);
        }
    }

    let mut index = 0;
    let mut stack = Vec::new();
    let mut indices: Vec<Option<usize>> = vec![None; n];
    let mut lowlink: Vec<usize> = vec![0; n];
    let mut on_stack: Vec<bool> = vec![false; n];
    let mut components: Vec<Vec<N>> = Vec::new();
    let mut work: Vec<(usize, usize, bool)> = Vec::new();

    for start in 0..n {
        if indices[start].is_some() {
            continue;
        }
        work.push((start, 0, false));
        while let Some((v, i, returned_from_child)) = work.pop() {
            if i == 0 && !returned_from_child {
                indices[v] = Some(index);
                lowlink[v] = index;
                index += 1;
                stack.push(v);
                on_stack[v] = true;
            }
            if i < adj[v].len() {
                let w = adj[v][i];
                if indices[w].is_none() {
                    work.push((v, i + 1, true));
                    work.push((w, 0, false));
                } else if on_stack[w] {
                    lowlink[v] = lowlink[v].min(indices[w].unwrap());
                    work.push((v, i + 1, false));
                } else {
                    work.push((v, i + 1, false));
                }
                continue;
            }
            if i > 0 && returned_from_child {
                let w = adj[v][i - 1];
                lowlink[v] = lowlink[v].min(lowlink[w]);
            }
            if indices[v] == Some(lowlink[v]) {
                let mut comp = Vec::new();
                loop {
                    let w = stack.pop().unwrap();
                    on_stack[w] = false;
                    comp.push(nodes[w].clone());
                    if w == v {
                        break;
                    }
                }
                components.push(comp);
            }
        }
    }

    components
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let c = scc::<&str>(Vec::new(), []);
        assert!(c.is_empty());
    }

    #[test]
    fn single_node() {
        let c = scc(["a"], []);
        assert_eq!(c.len(), 1);
        assert_eq!(&c[0], &["a"]);
    }

    #[test]
    fn by_key_sorts_component() {
        let nodes = ["C", "A", "B"];
        let edges = [("A", "B"), ("B", "C"), ("C", "A")];
        let c = scc_by_key(nodes, edges, |n| *n);
        assert_eq!(c.len(), 1);
        assert_eq!(&c[0], &["A", "B", "C"]);
    }
}
