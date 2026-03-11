use rust_stable_toposort::{toposort_layers, toposort_layers_by_key};

mod toposort_layers_normal {
    use super::*;

    #[test]
    fn empty() {
        let l = toposort_layers::<&str>(vec![], []).unwrap();
        assert!(l.is_empty());
    }

    #[test]
    fn single() {
        let l = toposort_layers(["a"], []).unwrap();
        assert_eq!(l, [vec!["a"]]);
    }

    #[test]
    fn two_independent() {
        let l = toposort_layers(["a", "b"], []).unwrap();
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].len(), 2);
    }

    #[test]
    fn chain_two() {
        let l = toposort_layers(["a", "b"], [("a", "b")]).unwrap();
        assert_eq!(l, [vec!["a"], vec!["b"]]);
    }

    #[test]
    fn chain_three() {
        let l = toposort_layers(["a", "b", "c"], [("a", "b"), ("b", "c")]).unwrap();
        assert_eq!(l.len(), 3);
        assert_eq!(l[0], ["a"]);
        assert_eq!(l[1], ["b"]);
        assert_eq!(l[2], ["c"]);
    }

    #[test]
    fn diamond() {
        let l = toposort_layers(["a", "b", "c"], [("a", "c"), ("b", "c")]).unwrap();
        assert_eq!(l.len(), 2);
        assert_eq!(l[0].len(), 2);
        assert!(l[0].contains(&"a") && l[0].contains(&"b"));
        assert_eq!(l[1], ["c"]);
    }

    #[test]
    fn cycle_err() {
        let r = toposort_layers(["a", "b"], [("a", "b"), ("b", "a")]);
        assert!(r.is_err());
    }

    #[test]
    fn double_diamond() {
        let nodes = ["a", "b", "c", "d", "e"];
        let edges = [("a", "c"), ("b", "c"), ("c", "d"), ("c", "e")];
        let l = toposort_layers(nodes, edges).unwrap();
        assert_eq!(l.len(), 3);
        assert_eq!(l[0].len(), 2);
        assert_eq!(l[1].len(), 1);
        assert_eq!(l[2].len(), 2);
    }

    #[test]
    fn five_chain() {
        let nodes = ["v1", "v2", "v3", "v4", "v5"];
        let edges = [("v1", "v2"), ("v2", "v3"), ("v3", "v4"), ("v4", "v5")];
        let l = toposort_layers(nodes, edges).unwrap();
        assert_eq!(l.len(), 5);
        for (i, layer) in l.iter().enumerate() {
            assert_eq!(layer.len(), 1);
            assert_eq!(layer[0], format!("v{}", i + 1));
        }
    }

    #[test]
    fn three_roots_one_sink() {
        let l = toposort_layers(["a", "b", "c", "d"], [("a", "d"), ("b", "d"), ("c", "d")]).unwrap();
        assert_eq!(l.len(), 2);
        assert_eq!(l[0].len(), 3);
        assert_eq!(l[1], ["d"]);
    }

    #[test]
    fn cycle_three_err() {
        let r = toposort_layers(["a", "b", "c"], [("a", "b"), ("b", "c"), ("c", "a")]);
        assert!(r.is_err());
    }

    #[test]
    fn layer_count_matches_depth() {
        let nodes = ["a", "b", "c", "d"];
        let edges = [("a", "b"), ("a", "c"), ("b", "d"), ("c", "d")];
        let l = toposort_layers(nodes, edges).unwrap();
        assert_eq!(l.len(), 3);
        assert_eq!(l[0], ["a"]);
        assert_eq!(l[1].len(), 2);
        assert_eq!(l[2], ["d"]);
    }

    #[test]
    fn all_nodes_in_layers() {
        let nodes = ["x", "y", "z"];
        let edges = [("x", "z"), ("y", "z")];
        let l = toposort_layers(nodes, edges).unwrap();
        let flat: std::collections::HashSet<_> = l.iter().flat_map(|x| x.iter()).collect();
        assert_eq!(flat.len(), 3);
    }

    #[test]
    fn edges_skip_unknown() {
        let l = toposort_layers(["a", "b"], [("a", "b"), ("a", "z")]).unwrap();
        assert_eq!(l.len(), 2);
        assert_eq!(l[0], ["a"]);
        assert_eq!(l[1], ["b"]);
    }

    #[test]
    fn many_independent_one_layer() {
        let nodes: Vec<String> = (0..25).map(|i| format!("n{}", i)).collect();
        let l = toposort_layers(nodes.iter().map(String::as_str), []).unwrap();
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].len(), 25);
    }
}

mod toposort_layers_by_key_normal {
    use super::*;

    #[test]
    fn empty() {
        let l = toposort_layers_by_key::<&str, i32>(vec![], [], |_| 0).unwrap();
        assert!(l.is_empty());
    }

    #[test]
    fn single() {
        let l = toposort_layers_by_key(["a"], [], |n| n.len()).unwrap();
        assert_eq!(l, [vec!["a"]]);
    }

    #[test]
    fn key_identity_diamond() {
        let l = toposort_layers_by_key(["B", "A", "C"], [("A", "C"), ("B", "C")], |n| *n).unwrap();
        assert_eq!(l[0], ["A", "B"]);
        assert_eq!(l[1], ["C"]);
    }

    #[test]
    fn key_length() {
        let l = toposort_layers_by_key(["aaa", "b", "cc"], [("b", "cc"), ("aaa", "cc")], |n| n.len()).unwrap();
        assert_eq!(l.len(), 2);
        assert!(l[0].contains(&"b") && l[0].contains(&"aaa"));
    }

    #[test]
    fn key_chain() {
        let l = toposort_layers_by_key([3, 1, 2], [(1, 2), (2, 3)], |&n| n).unwrap();
        assert_eq!(l.len(), 3);
        assert_eq!(l[0], [1]);
        assert_eq!(l[1], [2]);
        assert_eq!(l[2], [3]);
    }

    #[test]
    fn by_key_cycle_err() {
        let r = toposort_layers_by_key(["a", "b"], [("a", "b"), ("b", "a")], |n| *n);
        assert!(r.is_err());
    }

    #[test]
    fn key_constant() {
        let l = toposort_layers_by_key(["a", "b", "c"], [("a", "c"), ("b", "c")], |_| 0).unwrap();
        assert_eq!(l.len(), 2);
        assert_eq!(l[1], ["c"]);
    }

    #[test]
    fn key_tuple() {
        let nodes = [(1, 0), (0, 1)];
        let l = toposort_layers_by_key(nodes, [], |&n| n).unwrap();
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].len(), 2);
    }

    #[test]
    fn key_double_diamond() {
        let nodes = ["a", "b", "c", "d", "e"];
        let edges = [("a", "c"), ("b", "c"), ("c", "d"), ("c", "e")];
        let l = toposort_layers_by_key(nodes, edges, |n| n.as_bytes()[0]).unwrap();
        assert_eq!(l.len(), 3);
        assert_eq!(l[0].len(), 2);
        assert_eq!(l[2].len(), 2);
    }

    #[test]
    fn key_three_roots() {
        let l = toposort_layers_by_key(
            ["z", "a", "m", "t"],
            [("a", "t"), ("z", "t"), ("m", "t")],
            |n| *n,
        )
        .unwrap();
        assert_eq!(l.len(), 2);
        assert_eq!(l[1], ["t"]);
    }

    #[test]
    fn key_numeric() {
        let l = toposort_layers_by_key([10, 20, 30], [(10, 20), (20, 30)], |&n| n).unwrap();
        assert_eq!(l, [vec![10], vec![20], vec![30]]);
    }

    #[test]
    fn key_negated() {
        let l = toposort_layers_by_key(["aa", "b", "ccc"], [("b", "aa"), ("ccc", "aa")], |n| -(n.len() as i32)).unwrap();
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn key_option() {
        let nodes = [Some(1), None];
        let l = toposort_layers_by_key(nodes, [], |&n| n).unwrap();
        assert_eq!(l.len(), 1);
    }

    #[test]
    fn key_unicode() {
        let l = toposort_layers_by_key(["β", "α"], [("α", "β")], |n| *n).unwrap();
        assert_eq!(l, [vec!["α"], vec!["β"]]);
    }

    #[test]
    fn key_large_dag() {
        let nodes: Vec<i32> = (0..40).collect();
        let edges: Vec<(i32, i32)> = (0..39).map(|i| (i, i + 1)).collect();
        let l = toposort_layers_by_key(nodes, edges, |&n| n).unwrap();
        assert_eq!(l.len(), 40);
    }
}

mod toposort_layers_by_key_extra {
    use super::*;

    #[test]
    fn key_first_char() {
        let l = toposort_layers_by_key(["apple", "banana"], [("apple", "banana")], |n| n.chars().next().unwrap()).unwrap();
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn key_mod() {
        let l = toposort_layers_by_key([1, 2, 3, 4], [(1, 2), (2, 3), (3, 4)], |&n| n % 2).unwrap();
        assert_eq!(l.len(), 4);
    }

    #[test]
    fn key_str_ord() {
        let l = toposort_layers_by_key(["M", "A", "Z"], [("A", "M"), ("Z", "M")], |n| *n).unwrap();
        assert_eq!(l[1], ["M"]);
    }

    #[test]
    fn key_disconnected() {
        let l = toposort_layers_by_key(
            ["g1a", "g1b", "g2a", "g2b"],
            [("g1a", "g1b"), ("g2a", "g2b")],
            |n| n.chars().nth(1).unwrap(),
        )
        .unwrap();
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn key_empty_str() {
        let l = toposort_layers_by_key(["", "x"], [("", "x")], |n| n.len()).unwrap();
        assert_eq!(l, [vec![""], vec!["x"]]);
    }

    #[test]
    fn key_bool() {
        let nodes = [(true, 1), (false, 2)];
        let l = toposort_layers_by_key(nodes, [], |&(b, _)| b).unwrap();
        assert_eq!(l.len(), 1);
    }

    #[test]
    fn key_chain_reverse() {
        let l = toposort_layers_by_key([3, 2, 1], [(1, 2), (2, 3)], |&n| 4 - n).unwrap();
        assert_eq!(l.len(), 3);
    }

    #[test]
    fn key_complex_layers() {
        let nodes = (0..12).collect::<Vec<_>>();
        let edges = (0..11).map(|i| (i, i + 1)).collect::<Vec<_>>();
        let l = toposort_layers_by_key(nodes, edges, |&n| n / 3).unwrap();
        assert_eq!(l.len(), 12);
    }

    #[test]
    fn key_preserves_topo() {
        let l = toposort_layers_by_key(
            ["s", "t", "u", "v"],
            [("s", "u"), ("s", "v"), ("t", "u"), ("t", "v")],
            |n| n.as_bytes()[0],
        )
        .unwrap();
        assert_eq!(l.len(), 2);
        assert_eq!(l[1].len(), 2);
    }

    #[test]
    fn key_cycle_four_err() {
        let r = toposort_layers_by_key(
            ["a", "b", "c", "d"],
            [("a", "b"), ("b", "c"), ("c", "d"), ("d", "a")],
            |n| *n,
        );
        assert!(r.is_err());
    }

    #[test]
    fn key_tuple_three() {
        let nodes = [(1, 1), (1, 2), (2, 1)];
        let edges = [((1, 1), (2, 1)), ((1, 2), (2, 1))];
        let l = toposort_layers_by_key(nodes, edges, |&n| n).unwrap();
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn key_same_layer_ordered() {
        let l = toposort_layers_by_key(["c", "a", "b", "d"], [("a", "d"), ("b", "d"), ("c", "d")], |n| *n).unwrap();
        assert_eq!(l[0], ["a", "b", "c"]);
    }

    #[test]
    fn key_bytes() {
        let l = toposort_layers_by_key(["hi", "lo"], [("hi", "lo")], |n| n.as_bytes()).unwrap();
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn key_to_string() {
        let nodes = [1, 2, 3];
        let edges = [(1, 2), (2, 3)];
        let l = toposort_layers_by_key(nodes, edges, |&n| n.to_string()).unwrap();
        assert_eq!(l.len(), 3);
    }

    #[test]
    fn key_u64_bits() {
        let nodes = [1u64, 2, 3];
        let edges = [(1u64, 2), (2, 3)];
        let l = toposort_layers_by_key(nodes, edges, |&n| n).unwrap();
        assert_eq!(l.len(), 3);
    }
}
