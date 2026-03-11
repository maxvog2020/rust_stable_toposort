use stable_toposort::condensation::{toposort_scc, toposort_scc_by_key};
use stable_toposort::scc::scc;

mod toposort_scc_normal {
    use super::*;

    #[test]
    fn empty() {
        let s = toposort_scc::<&str>(vec![], []);
        assert!(s.is_empty());
    }

    #[test]
    fn single() {
        let s = toposort_scc(["a"], []);
        assert_eq!(s.len(), 1);
        assert_eq!(&s[0], &["a"]);
    }

    #[test]
    fn two_independent() {
        let s = toposort_scc(["a", "b"], []);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn chain_two() {
        let s = toposort_scc(["a", "b"], [("a", "b")]);
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], ["a"]);
        assert_eq!(s[1], ["b"]);
    }

    #[test]
    fn chain_three() {
        let s = toposort_scc(["a", "b", "c"], [("a", "b"), ("b", "c")]);
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn two_cycle_one_scc() {
        let s = toposort_scc(["a", "b"], [("a", "b"), ("b", "a")]);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].len(), 2);
    }

    #[test]
    fn cycle_three() {
        let s = toposort_scc(["a", "b", "c"], [("a", "b"), ("b", "c"), ("c", "a")]);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].len(), 3);
    }

    #[test]
    fn two_cycles_two_sccs() {
        let s = toposort_scc(
            ["a", "b", "x", "y"],
            [("a", "b"), ("b", "a"), ("x", "y"), ("y", "x")],
        );
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn dag_all_singleton() {
        let s = toposort_scc(["a", "b", "c"], [("a", "b"), ("b", "c")]);
        assert_eq!(s.len(), 3);
        for c in &s {
            assert_eq!(c.len(), 1);
        }
    }

    #[test]
    fn scc_count_matches_condensation() {
        let nodes = ["a", "b", "c", "d"];
        let edges = [("a", "b"), ("b", "a"), ("c", "d")];
        let s = toposort_scc(nodes, edges);
        let comps = scc(nodes, edges);
        assert_eq!(s.len(), comps.len());
    }

    #[test]
    fn all_nodes_present() {
        let s = toposort_scc(["p", "q", "r"], [("p", "q"), ("q", "p")]);
        let flat: std::collections::HashSet<_> = s.iter().flat_map(|x| x.iter()).collect();
        assert_eq!(flat.len(), 3);
    }

    #[test]
    fn five_cycle() {
        let nodes = ["a", "b", "c", "d", "e"];
        let edges = [("a", "b"), ("b", "c"), ("c", "d"), ("d", "e"), ("e", "a")];
        let s = toposort_scc(nodes, edges);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].len(), 5);
    }

    #[test]
    fn component_plus_dag() {
        let s = toposort_scc(
            ["x", "a", "b", "c"],
            [("a", "b"), ("b", "c"), ("c", "a")],
        );
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn edges_skip_unknown() {
        let s = toposort_scc(["a", "b"], [("a", "b"), ("a", "z")]);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn many_singletons() {
        let nodes: Vec<String> = (0..25).map(|i| format!("n{}", i)).collect();
        let s = toposort_scc(nodes.iter().map(String::as_str), []);
        assert_eq!(s.len(), 25);
    }
}

mod toposort_scc_by_key_normal {
    use super::*;

    #[test]
    fn empty() {
        let s = toposort_scc_by_key::<&str, i32>(vec![], [], |_| 0);
        assert!(s.is_empty());
    }

    #[test]
    fn single() {
        let s = toposort_scc_by_key(["a"], [], |n| n.len());
        assert_eq!(s.len(), 1);
        assert_eq!(&s[0], &["a"]);
    }

    #[test]
    fn key_sorts_components() {
        let s = toposort_scc_by_key(
            ["c", "a", "b"],
            [("a", "b"), ("b", "c")],
            |n| *n,
        );
        assert_eq!(s.len(), 3);
        assert_eq!(s[0], ["a"]);
        assert_eq!(s[1], ["b"]);
        assert_eq!(s[2], ["c"]);
    }

    #[test]
    fn key_cycle_sorted() {
        let s = toposort_scc_by_key(
            ["c", "a", "b"],
            [("a", "b"), ("b", "c"), ("c", "a")],
            |n| *n,
        );
        assert_eq!(s.len(), 1);
        assert_eq!(&s[0], &["a", "b", "c"]);
    }

    #[test]
    fn key_numeric() {
        let s = toposort_scc_by_key([3, 1, 2], [(1, 2), (2, 3)], |&n| n);
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn key_length() {
        let s = toposort_scc_by_key(["aaa", "b", "cc"], [("b", "cc"), ("aaa", "cc")], |n| n.len());
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn key_two_cycles() {
        let s = toposort_scc_by_key(
            ["b", "a", "y", "x"],
            [("a", "b"), ("b", "a"), ("x", "y"), ("y", "x")],
            |n| *n,
        );
        assert_eq!(s.len(), 2);
        for c in &s {
            assert_eq!(c.len(), 2);
            assert!(c[0] < c[1]);
        }
    }

    #[test]
    fn key_unicode() {
        let s = toposort_scc_by_key(
            ["γ", "α", "β"],
            [("α", "β"), ("β", "γ"), ("γ", "α")],
            |n| *n,
        );
        assert_eq!(s.len(), 1);
        assert_eq!(&s[0], &["α", "β", "γ"]);
    }

    #[test]
    fn key_tuple() {
        let nodes = [(2, 2), (1, 1)];
        let edges: [((i32, i32), (i32, i32)); 0] = [];
        let s = toposort_scc_by_key(nodes, edges, |&n| n);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn key_option() {
        let nodes = [Some(1), None];
        let edges: [(Option<i32>, Option<i32>); 0] = [];
        let s = toposort_scc_by_key(nodes, edges, |&n| n);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn key_constant() {
        let s = toposort_scc_by_key(["a", "b"], [("a", "b"), ("b", "a")], |_| 0);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].len(), 2);
    }

    #[test]
    fn key_reverse() {
        let s = toposort_scc_by_key([3, 2, 1], [(1, 2), (2, 3)], |&n| std::cmp::Reverse(n));
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn key_mod() {
        let s = toposort_scc_by_key([4, 2, 6], [(2, 4), (4, 6)], |&n| n % 3);
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn key_large_dag() {
        let nodes: Vec<i32> = (0..20).collect();
        let edges: Vec<(i32, i32)> = (0..19).map(|i| (i, i + 1)).collect();
        let s = toposort_scc_by_key(nodes, edges, |&n| n);
        assert_eq!(s.len(), 20);
    }

    #[test]
    fn key_bytes() {
        let s = toposort_scc_by_key(["hi", "lo"], [("hi", "lo")], |n| n.as_bytes()[0]);
        assert_eq!(s.len(), 2);
    }
}

mod toposort_scc_by_key_extra {
    use super::*;

    #[test]
    fn key_first_char() {
        let s = toposort_scc_by_key(
            ["banana", "apple"],
            [("apple", "banana")],
            |n| n.chars().next().unwrap(),
        );
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn key_negated() {
        let s = toposort_scc_by_key([1, 2, 3], [(1, 2), (2, 3)], |&n| -n);
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn key_string() {
        let nodes = [String::from("a"), String::from("b")];
        let edges: [(String, String); 0] = [];
        let s = toposort_scc_by_key(nodes, edges, |n| n.len());
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn key_three_sccs() {
        let s = toposort_scc_by_key(
            ["c", "a", "b", "f", "d", "e"],
            [("a", "b"), ("b", "c"), ("c", "a"), ("d", "e"), ("e", "f"), ("f", "d")],
            |n| *n,
        );
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn key_bool() {
        let nodes = [(true, 1), (false, 2)];
        let edges: [((bool, i32), (bool, i32)); 0] = [];
        let s = toposort_scc_by_key(nodes, edges, |&(b, _)| b);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn key_to_string() {
        let s = toposort_scc_by_key([3, 1, 2], [(1, 2), (2, 3)], |&n| n.to_string());
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn key_complex() {
        let nodes = (0..10).collect::<Vec<_>>();
        let edges = (0..9).map(|i| (i, i + 1)).collect::<Vec<_>>();
        let s = toposort_scc_by_key(nodes, edges, |&n| n / 2);
        assert_eq!(s.len(), 10);
    }

    #[test]
    fn key_empty_str() {
        let s = toposort_scc_by_key(["", "x"], [("", "x")], |n| n.len());
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn key_preserves_topo() {
        let s = toposort_scc_by_key(
            ["s", "t", "u", "v"],
            [("s", "u"), ("s", "v"), ("t", "u"), ("t", "v")],
            |n| n.as_bytes()[0],
        );
        assert_eq!(s.len(), 4);
    }

    #[test]
    fn key_ord_chain() {
        let s = toposort_scc_by_key([10, 20, 30], [(10, 20), (20, 30)], |&n| n);
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn key_tuple_pair() {
        let nodes = [(1, 2), (3, 4)];
        let edges: [((i32, i32), (i32, i32)); 0] = [];
        let s = toposort_scc_by_key(nodes, edges, |&n| n.0);
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn key_same_count_as_scc() {
        let nodes = ["a", "b", "c"];
        let edges = [("a", "b"), ("b", "c")];
        let comps = scc(nodes, edges);
        let s = toposort_scc_by_key(nodes, edges, |n| *n);
        assert_eq!(s.len(), comps.len());
    }

    #[test]
    fn key_large_sorted() {
        let nodes: Vec<i32> = (0..30).rev().collect();
        let edges: Vec<(i32, i32)> = (0..29).map(|i| (30 - i, 29 - i)).collect();
        let s = toposort_scc_by_key(nodes, edges, |&n| n);
        assert_eq!(s.len(), 30);
    }

    #[test]
    fn key_identity() {
        let s = toposort_scc_by_key(["m", "n"], [("m", "n")], |n| *n);
        assert_eq!(s.len(), 2);
        assert_eq!(s[0], ["m"]);
        assert_eq!(s[1], ["n"]);
    }
}
