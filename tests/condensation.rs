use stable_toposort::{
    condensation, condensation_by_key, scc, stable_toposort,
};

mod condensation_normal {
    use super::*;

    #[test]
    fn empty() {
        let c = condensation::<&str>(vec![], []);
        assert!(c.components.is_empty());
        assert!(c.edges.is_empty());
    }

    #[test]
    fn single() {
        let c = condensation(["a"], []);
        assert_eq!(c.components.len(), 1);
        assert_eq!(&c.components[0], &["a"]);
        assert!(c.edges.is_empty());
    }

    #[test]
    fn two_independent() {
        let c = condensation(["a", "b"], []);
        assert_eq!(c.components.len(), 2);
        assert!(c.edges.is_empty());
    }

    #[test]
    fn chain_two() {
        let c = condensation(["a", "b"], [("a", "b")]);
        assert_eq!(c.components.len(), 2);
        assert_eq!(c.edges.len(), 1);
    }

    #[test]
    fn chain_three() {
        let c = condensation(["a", "b", "c"], [("a", "b"), ("b", "c")]);
        assert_eq!(c.components.len(), 3);
        assert_eq!(c.edges.len(), 2);
    }

    #[test]
    fn two_cycle_one_component() {
        let c = condensation(["a", "b"], [("a", "b"), ("b", "a")]);
        assert_eq!(c.components.len(), 1);
        assert_eq!(c.components[0].len(), 2);
        assert!(c.edges.is_empty());
    }

    #[test]
    fn cycle_three() {
        let c = condensation(["a", "b", "c"], [("a", "b"), ("b", "c"), ("c", "a")]);
        assert_eq!(c.components.len(), 1);
        assert_eq!(c.components[0].len(), 3);
    }

    #[test]
    fn two_cycles_two_components() {
        let c = condensation(
            ["a", "b", "x", "y"],
            [("a", "b"), ("b", "a"), ("x", "y"), ("y", "x")],
        );
        assert_eq!(c.components.len(), 2);
        assert!(c.edges.is_empty());
    }

    #[test]
    fn cycle_plus_sink() {
        let c = condensation(
            ["a", "b", "c"],
            [("a", "b"), ("b", "a")],
        );
        assert_eq!(c.components.len(), 2);
    }

    #[test]
    fn dag_condensation_identity() {
        let nodes = ["a", "b", "c"];
        let edges = [("a", "b"), ("b", "c")];
        let c = condensation(nodes, edges);
        assert_eq!(c.components.len(), 3);
        let order = stable_toposort(0..c.components.len(), c.edges.iter().copied()).unwrap();
        assert_eq!(order.len(), 3);
        let pos: std::collections::HashMap<_, _> = order.iter().enumerate().map(|(i, &j)| (j, i)).collect();
        for &(i, j) in &c.edges {
            assert!(pos[&i] < pos[&j]);
        }
    }

    #[test]
    fn component_edges_dag() {
        let c = condensation(
            ["a", "b", "c", "d"],
            [("a", "b"), ("b", "c"), ("c", "a"), ("c", "d")],
        );
        assert_eq!(c.components.len(), 2);
        assert_eq!(c.edges.len(), 1);
    }

    #[test]
    fn all_nodes_in_components() {
        let c = condensation(["p", "q", "r"], [("p", "q"), ("q", "p")]);
        let flat: std::collections::HashSet<_> = c.components.iter().flat_map(|x| x.iter()).collect();
        assert_eq!(flat.len(), 3);
    }

    #[test]
    fn five_cycle() {
        let nodes = ["a", "b", "c", "d", "e"];
        let edges = [("a", "b"), ("b", "c"), ("c", "d"), ("d", "e"), ("e", "a")];
        let c = condensation(nodes, edges);
        assert_eq!(c.components.len(), 1);
        assert_eq!(c.components[0].len(), 5);
    }

    #[test]
    fn edges_skip_unknown() {
        let c = condensation(["a", "b"], [("a", "b"), ("a", "z")]);
        assert_eq!(c.components.len(), 2);
    }

    #[test]
    fn many_singletons() {
        let nodes: Vec<String> = (0..20).map(|i| format!("n{}", i)).collect();
        let c = condensation(nodes.iter().map(String::as_str), []);
        assert_eq!(c.components.len(), 20);
    }
}

mod condensation_by_key_normal {
    use super::*;

    #[test]
    fn empty() {
        let c = condensation_by_key::<&str, i32>(vec![] as Vec<&str>, [], |_| 0);
        assert!(c.components.is_empty());
    }

    #[test]
    fn single() {
        let c = condensation_by_key(["a"], [], |n| n.len());
        assert_eq!(c.components.len(), 1);
        assert_eq!(&c.components[0], &["a"]);
    }

    #[test]
    fn key_sorts_cycle() {
        let c = condensation_by_key(
            ["c", "a", "b"],
            [("a", "b"), ("b", "c"), ("c", "a")],
            |n| *n,
        );
        assert_eq!(c.components.len(), 1);
        assert_eq!(&c.components[0], &["a", "b", "c"]);
    }

    #[test]
    fn key_numeric() {
        let c = condensation_by_key([3, 1, 2], [(1, 2), (2, 3), (3, 1)], |&n| n);
        assert_eq!(c.components.len(), 1);
        assert_eq!(&c.components[0], &[1, 2, 3]);
    }

    #[test]
    fn key_dag() {
        let c = condensation_by_key(["a", "b", "c"], [("a", "b"), ("b", "c")], |n| *n);
        assert_eq!(c.components.len(), 3);
    }

    #[test]
    fn key_two_components() {
        let c = condensation_by_key(
            ["b", "a", "y", "x"],
            [("a", "b"), ("b", "a"), ("x", "y"), ("y", "x")],
            |n| *n,
        );
        assert_eq!(c.components.len(), 2);
        for comp in &c.components {
            assert_eq!(comp.len(), 2);
        }
    }

    #[test]
    fn key_unicode() {
        let c = condensation_by_key(
            ["γ", "α", "β"],
            [("α", "β"), ("β", "γ"), ("γ", "α")],
            |n| *n,
        );
        assert_eq!(c.components.len(), 1);
        assert_eq!(&c.components[0], &["α", "β", "γ"]);
    }

    #[test]
    fn key_length() {
        let c = condensation_by_key(
            ["zz", "x", "yy"],
            [("x", "yy"), ("yy", "zz"), ("zz", "x")],
            |n| n.len(),
        );
        assert_eq!(c.components.len(), 1);
    }

    #[test]
    fn key_edges_unchanged() {
        let c = condensation_by_key(
            ["a", "b", "c"],
            [("a", "b"), ("b", "c")],
            |n| *n,
        );
        assert_eq!(c.edges.len(), 2);
    }

    #[test]
    fn key_tuple() {
        let nodes = [(2, 2), (1, 1)];
        let edges: [((i32, i32), (i32, i32)); 0] = [];
        let c = condensation_by_key(nodes, edges, |&n| n);
        assert_eq!(c.components.len(), 2);
    }

    #[test]
    fn key_option() {
        let nodes = [Some(1), None];
        let edges: [(Option<i32>, Option<i32>); 0] = [];
        let c = condensation_by_key(nodes, edges, |&n| n);
        assert_eq!(c.components.len(), 2);
    }

    #[test]
    fn key_large_cycle() {
        let nodes: Vec<i32> = (0..15).collect();
        let mut edges: Vec<(i32, i32)> = (0..14).map(|i| (i, i + 1)).collect();
        edges.push((14, 0));
        let c = condensation_by_key(nodes, edges, |&n| n);
        assert_eq!(c.components.len(), 1);
        assert_eq!(c.components[0].len(), 15);
    }

    #[test]
    fn key_mod() {
        let c = condensation_by_key([4, 2, 6], [(2, 4), (4, 6), (6, 2)], |&n| n % 3);
        assert_eq!(c.components.len(), 1);
    }

    #[test]
    fn key_constant() {
        let c = condensation_by_key(["a", "b"], [("a", "b"), ("b", "a")], |_| 0);
        assert_eq!(c.components.len(), 1);
        assert_eq!(c.components[0].len(), 2);
    }

    #[test]
    fn key_reverse() {
        let c = condensation_by_key([3, 2, 1], [(1, 2), (2, 3), (3, 1)], |&n| std::cmp::Reverse(n));
        assert_eq!(c.components.len(), 1);
    }
}

mod condensation_by_key_extra {
    use super::*;

    #[test]
    fn key_first_char() {
        let c = condensation_by_key(
            ["banana", "apple"],
            [("apple", "banana"), ("banana", "apple")],
            |n| n.chars().next().unwrap(),
        );
        assert_eq!(c.components.len(), 1);
    }

    #[test]
    fn key_negated() {
        let c = condensation_by_key([1, 2, 3], [(1, 2), (2, 3), (3, 1)], |&n| -n);
        assert_eq!(c.components.len(), 1);
    }

    #[test]
    fn key_string() {
        let nodes = [String::from("a"), String::from("b")];
        let edges: [(String, String); 0] = [];
        let c = condensation_by_key(nodes, edges, |n| n.len());
        assert_eq!(c.components.len(), 2);
    }

    #[test]
    fn key_three_cycles() {
        let c = condensation_by_key(
            ["a1", "a2", "b1", "b2", "c1", "c2"],
            [("a1", "a2"), ("a2", "a1"), ("b1", "b2"), ("b2", "b1"), ("c1", "c2"), ("c2", "c1")],
            |n| n.chars().next().unwrap(),
        );
        assert_eq!(c.components.len(), 3);
    }

    #[test]
    fn key_bool() {
        let nodes = [(true, 1), (false, 2)];
        let edges: [((bool, i32), (bool, i32)); 0] = [];
        let c = condensation_by_key(nodes, edges, |&(b, _)| b);
        assert_eq!(c.components.len(), 2);
    }

    #[test]
    fn key_to_string() {
        let c = condensation_by_key([3, 1, 2], [(1, 2), (2, 3), (3, 1)], |&n| n.to_string());
        assert_eq!(c.components.len(), 1);
    }

    #[test]
    fn key_complex() {
        let nodes = (0..8).collect::<Vec<_>>();
        let edges = (0..7).map(|i| (i, i + 1)).chain(std::iter::once((7, 0))).collect::<Vec<_>>();
        let c = condensation_by_key(nodes, edges, |&n| n / 2);
        assert_eq!(c.components.len(), 1);
    }

    #[test]
    fn key_empty_str() {
        let c = condensation_by_key(["", "x"], [("", "x"), ("x", "")], |n| n.len());
        assert_eq!(c.components.len(), 1);
    }

    #[test]
    fn key_bytes() {
        let c = condensation_by_key(["hi", "lo"], [("hi", "lo"), ("lo", "hi")], |n| n.as_bytes()[0]);
        assert_eq!(c.components.len(), 1);
    }

    #[test]
    fn key_ord_chain() {
        let c = condensation_by_key([10, 20, 30], [(10, 20), (20, 30)], |&n| n);
        assert_eq!(c.components.len(), 3);
    }

    #[test]
    fn key_tuple_pair() {
        let nodes = [(1, 2), (3, 4)];
        let edges: [((i32, i32), (i32, i32)); 0] = [];
        let c = condensation_by_key(nodes, edges, |&n| n.0);
        assert_eq!(c.components.len(), 2);
    }

    #[test]
    fn key_same_as_scc() {
        let nodes = ["z", "y", "x"];
        let edges = [("x", "y"), ("y", "z"), ("z", "x")];
        let comps = scc(nodes, edges);
        let c = condensation_by_key(nodes, edges, |n| *n);
        assert_eq!(c.components.len(), comps.len());
    }

    #[test]
    fn key_large_dag() {
        let nodes: Vec<i32> = (0..30).collect();
        let edges: Vec<(i32, i32)> = (0..29).map(|i| (i, i + 1)).collect();
        let c = condensation_by_key(nodes, edges, |&n| n);
        assert_eq!(c.components.len(), 30);
    }

    #[test]
    fn key_identity() {
        let c = condensation_by_key(["m", "n"], [("m", "n"), ("n", "m")], |n| *n);
        assert_eq!(c.components.len(), 1);
        assert_eq!(&c.components[0], &["m", "n"]);
    }
}
