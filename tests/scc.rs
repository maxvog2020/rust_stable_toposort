use stable_toposort::scc::{scc, scc_by_key};

mod scc_normal {
    use super::*;

    #[test]
    fn empty() {
        let c = scc::<&str>(vec![], []);
        assert!(c.is_empty());
    }

    #[test]
    fn single() {
        let c = scc(["a"], []);
        assert_eq!(c.len(), 1);
        assert_eq!(&c[0], &["a"]);
    }

    #[test]
    fn two_independent() {
        let c = scc(["a", "b"], []);
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn two_cycle() {
        let c = scc(["a", "b"], [("a", "b"), ("b", "a")]);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 2);
    }

    #[test]
    fn chain_three() {
        let c = scc(["a", "b", "c"], [("a", "b"), ("b", "c")]);
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn cycle_three() {
        let c = scc(["a", "b", "c"], [("a", "b"), ("b", "c"), ("c", "a")]);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 3);
    }

    #[test]
    fn two_components() {
        let c = scc(
            ["a", "b", "x", "y"],
            [("a", "b"), ("b", "a"), ("x", "y"), ("y", "x")],
        );
        assert_eq!(c.len(), 2);
        let sizes: std::collections::HashSet<_> = c.iter().map(|x| x.len()).collect();
        assert!(sizes.contains(&2));
    }

    #[test]
    fn dag_all_singleton() {
        let c = scc(["a", "b", "c"], [("a", "b"), ("b", "c")]);
        assert_eq!(c.len(), 3);
        for comp in &c {
            assert_eq!(comp.len(), 1);
        }
    }

    #[test]
    fn self_loop() {
        let c = scc(["a"], [("a", "a")]);
        assert_eq!(c.len(), 1);
        assert_eq!(&c[0], &["a"]);
    }

    #[test]
    fn cycle_four() {
        let c = scc(
            ["a", "b", "c", "d"],
            [("a", "b"), ("b", "c"), ("c", "d"), ("d", "a")],
        );
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 4);
    }

    #[test]
    fn component_plus_dag() {
        let c = scc(
            ["x", "a", "b", "c"],
            [("a", "b"), ("b", "c"), ("c", "a")],
        );
        assert_eq!(c.len(), 2);
        let has_cycle = c.iter().any(|comp| comp.len() == 3);
        assert!(has_cycle);
    }

    #[test]
    fn all_nodes_in_components() {
        let c = scc(["p", "q", "r"], [("p", "q"), ("q", "p")]);
        let flat: std::collections::HashSet<_> = c.iter().flat_map(|x| x.iter()).collect();
        assert_eq!(flat.len(), 3);
    }

    #[test]
    fn five_cycle() {
        let nodes = ["a", "b", "c", "d", "e"];
        let edges = [("a", "b"), ("b", "c"), ("c", "d"), ("d", "e"), ("e", "a")];
        let c = scc(nodes, edges);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 5);
    }

    #[test]
    fn edges_skip_unknown() {
        let c = scc(["a", "b"], [("a", "b"), ("a", "z"), ("z", "a")]);
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn many_singletons() {
        let nodes: Vec<String> = (0..30).map(|i| format!("n{}", i)).collect();
        let c = scc(nodes.iter().map(String::as_str), []);
        assert_eq!(c.len(), 30);
    }
}

mod scc_by_key_normal {
    use super::*;

    #[test]
    fn empty() {
        let c = scc_by_key::<&str, i32>(vec![], [], |_| 0);
        assert!(c.is_empty());
    }

    #[test]
    fn single() {
        let c = scc_by_key(["a"], [], |n| n.len());
        assert_eq!(c.len(), 1);
        assert_eq!(&c[0], &["a"]);
    }

    #[test]
    fn key_sorts_cycle() {
        let c = scc_by_key(["c", "a", "b"], [("a", "b"), ("b", "c"), ("c", "a")], |n| *n);
        assert_eq!(c.len(), 1);
        assert_eq!(&c[0], &["a", "b", "c"]);
    }

    #[test]
    fn key_numeric() {
        let c = scc_by_key([3, 1, 2], [(1, 2), (2, 3), (3, 1)], |&n| n);
        assert_eq!(c.len(), 1);
        assert_eq!(&c[0], &[1, 2, 3]);
    }

    #[test]
    fn key_length() {
        let c = scc_by_key(["zz", "x", "yy"], [("x", "yy"), ("yy", "zz"), ("zz", "x")], |n| n.len());
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 3);
    }

    #[test]
    fn key_reverse() {
        let c = scc_by_key([1, 2, 3], [(1, 2), (2, 3), (3, 1)], |&n| std::cmp::Reverse(n));
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 3);
    }

    #[test]
    fn key_tuple() {
        let nodes = [(2, 2), (1, 1), (3, 3)];
        let edges = [((1, 1), (2, 2)), ((2, 2), (3, 3)), ((3, 3), (1, 1))];
        let c = scc_by_key(nodes, edges, |&n| n);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 3);
    }

    #[test]
    fn key_dag_singletons() {
        let c = scc_by_key(["a", "b", "c"], [("a", "b"), ("b", "c")], |n| *n);
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn key_two_components_sorted() {
        let c = scc_by_key(
            ["b", "a", "y", "x"],
            [("a", "b"), ("b", "a"), ("x", "y"), ("y", "x")],
            |n| *n,
        );
        assert_eq!(c.len(), 2);
        for comp in &c {
            assert_eq!(comp.len(), 2);
            assert!(comp[0] < comp[1]);
        }
    }

    #[test]
    fn key_unicode() {
        let c = scc_by_key(["γ", "α", "β"], [("α", "β"), ("β", "γ"), ("γ", "α")], |n| *n);
        assert_eq!(c.len(), 1);
        assert_eq!(&c[0], &["α", "β", "γ"]);
    }

    #[test]
    fn key_option() {
        let nodes = [Some(2), None, Some(1)];
        let edges: [(Option<i32>, Option<i32>); 0] = [];
        let c = scc_by_key(nodes, edges, |&n| n);
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn key_constant() {
        let c = scc_by_key(["a", "b", "c"], [("a", "b"), ("b", "a")], |_| 0);
        assert_eq!(c.len(), 2);
        assert_eq!(c.iter().map(|x| x.len()).sum::<usize>(), 3);
    }

    #[test]
    fn key_large_cycle() {
        let nodes: Vec<i32> = (0..20).collect();
        let mut edges: Vec<(i32, i32)> = (0..19).map(|i| (i, i + 1)).collect();
        edges.push((19, 0));
        let c = scc_by_key(nodes, edges, |&n| n);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].len(), 20);
    }

    #[test]
    fn key_mod() {
        let c = scc_by_key([4, 2, 6], [(2, 4), (4, 6), (6, 2)], |&n| n % 3);
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn key_bytes() {
        let c = scc_by_key(["hi", "lo"], [("hi", "lo"), ("lo", "hi")], |n| n.as_bytes()[0]);
        assert_eq!(c.len(), 1);
    }
}

mod scc_by_key_extra {
    use super::*;

    #[test]
    fn key_first_char() {
        let c = scc_by_key(["banana", "apple"], [("apple", "banana"), ("banana", "apple")], |n| n.chars().next().unwrap());
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn key_negated() {
        let c = scc_by_key([1, 2, 3], [(1, 2), (2, 3), (3, 1)], |&n| -n);
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn key_string() {
        let nodes = [String::from("b"), String::from("a")];
        let edges: [(String, String); 0] = [];
        let c = scc_by_key(nodes, edges, |n| n.len());
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn key_three_components() {
        let c = scc_by_key(
            ["c", "a", "b", "f", "d", "e"],
            [("a", "b"), ("b", "c"), ("c", "a"), ("d", "e"), ("e", "f"), ("f", "d")],
            |n| *n,
        );
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn key_bool() {
        let nodes = [(true, 1), (false, 2)];
        let edges: [((bool, i32), (bool, i32)); 0] = [];
        let c = scc_by_key(nodes, edges, |&(b, _)| b);
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn key_to_string() {
        let c = scc_by_key([3, 1, 2], [(1, 2), (2, 3), (3, 1)], |&n| n.to_string());
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn key_complex() {
        let nodes = (0..10).collect::<Vec<_>>();
        let edges = (0..9).map(|i| (i, i + 1)).chain(std::iter::once((9, 0))).collect::<Vec<_>>();
        let c = scc_by_key(nodes, edges, |&n| n / 2);
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn key_empty_str() {
        let c = scc_by_key(["", "x"], [("", "x"), ("x", "")], |n| n.len());
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn key_multiple_cycles() {
        let c = scc_by_key(
            ["a1", "a2", "b1", "b2"],
            [("a1", "a2"), ("a2", "a1"), ("b1", "b2"), ("b2", "b1")],
            |n| n.chars().next().unwrap(),
        );
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn key_preserves_components() {
        let c = scc_by_key(["z", "y", "x"], [("x", "y"), ("y", "z"), ("z", "x")], |n| *n);
        assert_eq!(c.len(), 1);
        assert!(c[0].contains(&"x") && c[0].contains(&"y") && c[0].contains(&"z"));
    }

    #[test]
    fn key_ord_chain() {
        let c = scc_by_key([10, 20, 30], [(10, 20), (20, 30)], |&n| n);
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn key_tuple_pair() {
        let nodes = [(1, 2), (3, 4)];
        let edges: [((i32, i32), (i32, i32)); 0] = [];
        let c = scc_by_key(nodes, edges, |&n| n.0);
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn key_same_key_different_components() {
        let c = scc_by_key(
            ["a", "b", "c", "d"],
            [("a", "b"), ("b", "a"), ("c", "d"), ("d", "c")],
            |n| n.len(),
        );
        assert_eq!(c.len(), 2);
    }

    #[test]
    fn key_large_sorted() {
        let nodes: Vec<i32> = (0..25).rev().collect();
        let edges: Vec<(i32, i32)> = (0..24).map(|i| (24 - i, 23 - i)).collect();
        let c = scc_by_key(nodes, edges, |&n| n);
        assert_eq!(c.len(), 25);
    }

    #[test]
    fn key_identity() {
        let c = scc_by_key(["m", "n"], [("m", "n"), ("n", "m")], |n| *n);
        assert_eq!(c.len(), 1);
        assert_eq!(&c[0], &["m", "n"]);
    }
}
