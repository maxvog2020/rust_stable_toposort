use stable_toposort::cycle::CycleError;
use stable_toposort::toposort::{toposort, toposort_by_key, toposort_indices, toposort_indices_with_keys};

mod toposort_normal {
    use super::*;

    #[test]
    fn empty() {
        let o = toposort::<&str>(vec![], []).unwrap();
        assert!(o.is_empty());
    }

    #[test]
    fn single() {
        assert_eq!(toposort(["a"], []).unwrap(), ["a"]);
    }

    #[test]
    fn two_independent() {
        assert_eq!(toposort(["a", "b"], []).unwrap(), ["a", "b"]);
    }

    #[test]
    fn two_ordered() {
        assert_eq!(
            toposort(["a", "b"], [("a", "b")]).unwrap(),
            ["a", "b"]
        );
    }

    #[test]
    fn two_cycle() {
        let r = toposort(["a", "b"], [("a", "b"), ("b", "a")]);
        assert!(matches!(r, Err(CycleError { .. })));
        let Err(e) = r else { unreachable!() };
        assert!(e.cycle.len() >= 2);
    }

    #[test]
    fn chain_three() {
        assert_eq!(
            toposort(["a", "b", "c"], [("a", "b"), ("b", "c")]).unwrap(),
            ["a", "b", "c"]
        );
    }

    #[test]
    fn diamond_stable() {
        let o = toposort(["a", "b", "c"], [("a", "c"), ("b", "c")]).unwrap();
        assert_eq!(o[0], "a");
        assert_eq!(o[1], "b");
        assert_eq!(o[2], "c");
    }

    #[test]
    fn diamond_reverse_order() {
        let o = toposort(["b", "a", "c"], [("a", "c"), ("b", "c")]).unwrap();
        assert_eq!(o, ["b", "a", "c"]);
    }

    #[test]
    fn cycle_three() {
        let r = toposort(["a", "b", "c"], [("a", "b"), ("b", "c"), ("c", "a")]);
        let Err(e) = r else { panic!("expected cycle") };
        assert!(e.cycle.len() >= 2);
    }

    #[test]
    fn disconnected_two_dags() {
        let o = toposort(
            ["x", "y", "a", "b"],
            [("a", "b")],
        )
        .unwrap();
        assert_eq!(o.iter().filter(|&&n| n == "a" || n == "b").cloned().collect::<Vec<_>>(), ["a", "b"]);
        assert!(o.iter().position(|&x| x == "a").unwrap() < o.iter().position(|&x| x == "b").unwrap());
    }

    #[test]
    fn five_node_chain() {
        let nodes = ["v1", "v2", "v3", "v4", "v5"];
        let edges = [("v1", "v2"), ("v2", "v3"), ("v3", "v4"), ("v4", "v5")];
        assert_eq!(toposort(nodes, edges).unwrap(), ["v1", "v2", "v3", "v4", "v5"]);
    }

    #[test]
    fn double_diamond() {
        let nodes = ["a", "b", "c", "d", "e"];
        let edges = [("a", "c"), ("b", "c"), ("c", "d"), ("c", "e")];
        let o = toposort(nodes, edges).unwrap();
        let pos = |n: &str| o.iter().position(|&x| x == n).unwrap();
        assert!(pos("a") < pos("c") && pos("b") < pos("c"));
        assert!(pos("c") < pos("d") && pos("c") < pos("e"));
    }

    #[test]
    fn cycle_contains_all_in_cycle() {
        let r = toposort(["a", "b", "c", "d"], [("a", "b"), ("b", "c"), ("c", "d"), ("d", "a")]);
        let Err(e) = r else { panic!() };
        let set: std::collections::HashSet<_> = e.cycle.iter().collect();
        assert!(set.len() >= 2);
    }

    #[test]
    fn edges_skip_unknown_nodes() {
        let o = toposort(["a", "b"], [("a", "b"), ("a", "z"), ("z", "q")]).unwrap();
        assert_eq!(o, ["a", "b"]);
    }

    #[test]
    fn many_independent() {
        let nodes: Vec<String> = (0..20).map(|i| format!("n{}", i)).collect();
        let o = toposort(nodes.iter().map(String::as_str), []).unwrap();
        assert_eq!(o.len(), 20);
        for (i, s) in o.iter().enumerate() {
            assert_eq!(*s, format!("n{}", i));
        }
    }
}

mod toposort_by_key_normal {
    use super::*;

    #[test]
    fn empty() {
        let o = toposort_by_key::<&str, i32>(vec![], [], |_| 0).unwrap();
        assert!(o.is_empty());
    }

    #[test]
    fn single() {
        assert_eq!(toposort_by_key(["a"], [], |n| n.len()).unwrap(), ["a"]);
    }

    #[test]
    fn key_identity_alphabetic() {
        let nodes = ["c", "a", "b"];
        let edges = [("a", "c"), ("b", "c")];
        let o = toposort_by_key(nodes, edges, |n| *n).unwrap();
        assert_eq!(o, ["a", "b", "c"]);
    }

    #[test]
    fn key_length() {
        let nodes = ["aaa", "b", "cc"];
        let edges = [("b", "cc"), ("aaa", "cc")];
        let o = toposort_by_key(nodes, edges, |n| n.len()).unwrap();
        assert_eq!(o[0], "b");
        let pos = |n: &str| o.iter().position(|&x| x == n).unwrap();
        assert!(pos("b") < pos("cc") && pos("aaa") < pos("cc"));
    }

    #[test]
    fn key_reverse_ord() {
        let nodes = [3, 1, 2];
        let edges = [(1, 2), (2, 3)];
        let o = toposort_by_key(nodes, edges, |&n| std::cmp::Reverse(n)).unwrap();
        assert_eq!(o.len(), 3);
        assert!(o.iter().position(|&x| x == 1).unwrap() < o.iter().position(|&x| x == 2).unwrap());
        assert!(o.iter().position(|&x| x == 2).unwrap() < o.iter().position(|&x| x == 3).unwrap());
    }

    #[test]
    fn key_tuple_primary_secondary() {
        let nodes = ["a2", "b1", "a1"];
        let edges = [("a1", "b1"), ("a2", "b1")];
        let o = toposort_by_key(nodes, edges, |n| (n.chars().next().unwrap(), n.chars().nth(1).unwrap()));
        let o = o.unwrap();
        let pos = |n: &str| o.iter().position(|&x| x == n).unwrap();
        assert!(pos("a1") < pos("b1") && pos("a2") < pos("b1"));
    }

    #[test]
    fn by_key_cycle_error() {
        let r = toposort_by_key(["x", "y"], [("x", "y"), ("y", "x")], |n| *n);
        assert!(r.is_err());
    }

    #[test]
    fn key_constant_all_same() {
        let nodes = ["a", "b", "c"];
        let edges = [("a", "c"), ("b", "c")];
        let o = toposort_by_key(nodes, edges, |_| 0).unwrap();
        assert_eq!(o.len(), 3);
        assert!(o.contains(&"a") && o.contains(&"b") && o.contains(&"c"));
    }

    #[test]
    fn key_numeric_chain() {
        let nodes = [10, 20, 30];
        let edges = [(10, 20), (20, 30)];
        let o = toposort_by_key(nodes, edges, |&n| n).unwrap();
        assert_eq!(o, [10, 20, 30]);
    }

    #[test]
    fn key_negated_ordering() {
        let nodes = ["first", "second", "third"];
        let edges = [("first", "second"), ("second", "third")];
        let o = toposort_by_key(nodes, edges, |n| -(n.len() as i32)).unwrap();
        let pos = |n: &str| o.iter().position(|&x| x == n).unwrap();
        assert!(pos("first") < pos("second") && pos("second") < pos("third"));
    }

    #[test]
    fn by_key_large_dag() {
        let nodes: Vec<i32> = (0..50).collect();
        let edges: Vec<(i32, i32)> = (0..49).map(|i| (i, i + 1)).collect();
        let o = toposort_by_key(nodes, edges, |&n| n).unwrap();
        assert_eq!(o, (0..50).collect::<Vec<_>>());
    }

    #[test]
    fn key_mod_3_then_value() {
        let nodes = [2, 4, 6, 1, 3, 5];
        let edges = [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)];
        let o = toposort_by_key(nodes, edges, |&n| (n % 3, n)).unwrap();
        assert_eq!(o, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn key_str_ord_diamond() {
        let nodes = ["M", "Z", "A"];
        let edges = [("A", "M"), ("Z", "M")];
        let o = toposort_by_key(nodes, edges, |n| *n).unwrap();
        assert_eq!(o[2], "M");
        assert!(o.contains(&"A") && o.contains(&"Z"));
    }

    #[test]
    fn by_key_preserves_topo() {
        let nodes = ["s", "t", "u", "v"];
        let edges = [("s", "u"), ("s", "v"), ("t", "u"), ("t", "v")];
        let o = toposort_by_key(nodes, edges, |n| n.as_bytes()[0]).unwrap();
        let pos = |n: &str| o.iter().position(|&x| x == n).unwrap();
        assert!(pos("s") < pos("u") && pos("s") < pos("v"));
        assert!(pos("t") < pos("u") && pos("t") < pos("v"));
    }

    #[test]
    fn key_bool_false_first() {
        let nodes = [(true, "b"), (false, "a")];
        let edges = [((false, "a"), (true, "b"))];
        let o = toposort_by_key(nodes, edges, |&(b, _)| b).unwrap();
        assert_eq!(o[0].0, false);
        assert_eq!(o[1].0, true);
    }
}

mod toposort_by_key_extra {
    use super::*;

    #[test]
    fn key_first_char() {
        let nodes = ["apple", "banana", "apricot"];
        let edges = [("apple", "banana"), ("apricot", "banana")];
        let o = toposort_by_key(nodes, edges, |n| n.chars().next().unwrap()).unwrap();
        let pos = |n: &str| o.iter().position(|&x| x == n).unwrap();
        assert!(pos("apple") < pos("banana") && pos("apricot") < pos("banana"));
    }

    #[test]
    fn key_option_none_first() {
        let nodes = [Some(2), None, Some(1)];
        let edges = [(None, Some(1)), (Some(2), Some(1))];
        let o = toposort_by_key(nodes, edges, |&n| n).unwrap();
        assert_eq!(o[0], None);
    }

    #[test]
    fn key_ord_chain_reverse_key() {
        let nodes = [3, 2, 1];
        let edges = [(1, 2), (2, 3)];
        let o = toposort_by_key(nodes, edges, |&n| 4 - n).unwrap();
        assert_eq!(o, [1, 2, 3]);
    }

    #[test]
    fn key_multiple_roots_ordered_by_key() {
        let nodes = ["z", "a", "m"];
        let edges = [("a", "m"), ("z", "m")];
        let o = toposort_by_key(nodes, edges, |n| *n).unwrap();
        assert_eq!(o[2], "m");
        assert!(o.iter().position(|&x| x == "a").unwrap() < o.iter().position(|&x| x == "m").unwrap());
        assert!(o.iter().position(|&x| x == "z").unwrap() < o.iter().position(|&x| x == "m").unwrap());
    }

    #[test]
    fn key_empty_str() {
        let nodes = ["", "x"];
        let edges = [("", "x")];
        let o = toposort_by_key(nodes, edges, |n| n.len()).unwrap();
        assert_eq!(o, ["", "x"]);
    }

    #[test]
    fn key_unicode() {
        let nodes = ["β", "α", "γ"];
        let edges = [("α", "γ"), ("β", "γ")];
        let o = toposort_by_key(nodes, edges, |n| *n).unwrap();
        assert_eq!(o, ["α", "β", "γ"]);
    }

    #[test]
    fn key_double_diamond() {
        let nodes = ["a", "b", "c", "d", "e"];
        let edges = [("a", "c"), ("b", "c"), ("c", "d"), ("c", "e")];
        let o = toposort_by_key(nodes, edges, |n| n.as_bytes()[0]).unwrap();
        assert_eq!(o, ["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn key_three_way_merge() {
        let nodes = ["1", "2", "3", "4"];
        let edges = [("1", "4"), ("2", "4"), ("3", "4")];
        let o = toposort_by_key(nodes, edges, |n| n.parse::<i32>().unwrap()).unwrap();
        assert_eq!(o, ["1", "2", "3", "4"]);
    }

    #[test]
    fn key_cycle_four() {
        let r = toposort_by_key(
            ["a", "b", "c", "d"],
            [("a", "b"), ("b", "c"), ("c", "d"), ("d", "a")],
            |n| *n,
        );
        assert!(r.is_err());
    }

    #[test]
    fn key_disconnected_groups() {
        let nodes = ["g1a", "g1b", "g2a", "g2b"];
        let edges = [("g1a", "g1b"), ("g2a", "g2b")];
        let o = toposort_by_key(nodes, edges, |n| n.chars().nth(1).unwrap()).unwrap();
        assert_eq!(o.len(), 4);
        let pos = |n: &str| o.iter().position(|&x| x == n).unwrap();
        assert!(pos("g1a") < pos("g1b") && pos("g2a") < pos("g2b"));
    }

    #[test]
    fn key_tuple_three_levels() {
        let nodes = [(1, 1), (1, 2), (2, 1)];
        let edges = [((1, 1), (2, 1)), ((1, 2), (2, 1))];
        let o = toposort_by_key(nodes, edges, |&n| n).unwrap();
        assert_eq!(o.len(), 3);
    }

    #[test]
    fn key_hash_stable() {
        let nodes = ["x"; 5];
        let edges: [(&str, &str); 0] = [];
        let o = toposort_by_key(nodes, edges, |n| n.len()).unwrap();
        assert_eq!(o.len(), 5);
    }

    #[test]
    fn key_complex_graph() {
        let nodes = (0..15).collect::<Vec<_>>();
        let edges = (0..14).map(|i| (i, i + 1)).collect::<Vec<_>>();
        let o = toposort_by_key(nodes, edges, |&n| n % 5).unwrap();
        assert_eq!(o, (0..15).collect::<Vec<_>>());
    }

    #[test]
    fn key_borrow() {
        let nodes = ["hello", "world"];
        let edges = [("hello", "world")];
        let o = toposort_by_key(nodes, edges, |n| n.as_bytes()).unwrap();
        assert_eq!(o, ["hello", "world"]);
    }

    #[test]
    fn key_ord_float_like() {
        let nodes = [1, 2, 3];
        let edges = [(1, 2), (2, 3)];
        let o = toposort_by_key(nodes, edges, |&n| (n as f64).to_bits()).unwrap();
        assert_eq!(o, [1, 2, 3]);
    }

    #[test]
    fn key_string_len() {
        let nodes = [String::from("a"), String::from("b")];
        let edges: [(String, String); 0] = [];
        let o = toposort_by_key(nodes, edges, |n| n.len()).unwrap();
        assert_eq!(o, ["a", "b"]);
    }
}

mod toposort_indices_tests {
    use super::*;

    #[test]
    fn empty() {
        let o = toposort_indices([], 0).unwrap();
        assert!(o.is_empty());
    }

    #[test]
    fn single() {
        assert_eq!(toposort_indices([], 1).unwrap(), [0]);
    }

    #[test]
    fn two_independent() {
        assert_eq!(toposort_indices([], 2).unwrap(), [0, 1]);
    }

    #[test]
    fn two_ordered() {
        assert_eq!(toposort_indices([(0, 1)], 2).unwrap(), [0, 1]);
    }

    #[test]
    fn two_cycle() {
        let r = toposort_indices([(0, 1), (1, 0)], 2);
        assert!(matches!(r, Err(CycleError { .. })));
        let Err(e) = r else { unreachable!() };
        assert!(e.cycle.len() >= 2);
    }

    #[test]
    fn chain_three() {
        assert_eq!(
            toposort_indices([(0, 1), (1, 2)], 3).unwrap(),
            [0, 1, 2]
        );
    }

    #[test]
    fn diamond_stable() {
        let o = toposort_indices([(0, 2), (1, 2)], 3).unwrap();
        assert_eq!(o, [0, 1, 2]);
    }

    #[test]
    fn cycle_three() {
        let r = toposort_indices([(0, 1), (1, 2), (2, 0)], 3);
        let Err(e) = r else { panic!("expected cycle") };
        assert!(e.cycle.len() >= 2);
    }

    #[test]
    fn disconnected_two_components() {
        let o = toposort_indices([(0, 1)], 4).unwrap();
        assert_eq!(o.len(), 4);
        assert!(o.iter().position(|&x| x == 0).unwrap() < o.iter().position(|&x| x == 1).unwrap());
    }

    #[test]
    fn five_node_chain() {
        let edges = [(0, 1), (1, 2), (2, 3), (3, 4)];
        assert_eq!(toposort_indices(edges, 5).unwrap(), [0, 1, 2, 3, 4]);
    }

    #[test]
    fn double_diamond() {
        let edges = [(0, 2), (1, 2), (2, 3), (2, 4)];
        let o = toposort_indices(edges, 5).unwrap();
        let pos = |i| o.iter().position(|&x| x == i).unwrap();
        assert!(pos(0) < pos(2) && pos(1) < pos(2));
        assert!(pos(2) < pos(3) && pos(2) < pos(4));
    }

    #[test]
    fn large_size_no_edges() {
        let o = toposort_indices(vec![] as Vec<(usize, usize)>, 20).unwrap();
        assert_eq!(o, (0..20).collect::<Vec<_>>());
    }

    #[test]
    fn cycle_contains_indices() {
        let r = toposort_indices([(0, 1), (1, 2), (2, 3), (3, 0)], 4);
        let Err(e) = r else { panic!() };
        let set: std::collections::HashSet<_> = e.cycle.iter().collect();
        assert!(set.len() >= 2);
    }
}

mod toposort_indices_with_keys_tests {
    use super::*;

    #[test]
    fn empty() {
        let keys: [i32; 0] = [];
        let o = toposort_indices_with_keys([], &keys).unwrap();
        assert!(o.is_empty());
    }

    #[test]
    fn single() {
        let keys = [10];
        assert_eq!(toposort_indices_with_keys([], &keys).unwrap(), [0]);
    }

    #[test]
    fn two_independent_ordered_by_key() {
        let keys = [20, 10];
        let o = toposort_indices_with_keys([], &keys).unwrap();
        assert_eq!(o, [1, 0]);
    }

    #[test]
    fn two_ordered() {
        let keys = ["a", "b"];
        assert_eq!(toposort_indices_with_keys([(0, 1)], &keys).unwrap(), [0, 1]);
    }

    #[test]
    fn two_cycle() {
        let keys = [1, 2];
        let r = toposort_indices_with_keys([(0, 1), (1, 0)], &keys);
        assert!(r.is_err());
    }

    #[test]
    fn diamond_ordered_by_key() {
        let keys = ["C", "A", "B"];
        let edges = [(0, 2), (1, 2)];
        let o = toposort_indices_with_keys(edges, &keys).unwrap();
        assert_eq!(o, [1, 0, 2]);
    }

    #[test]
    fn chain_three() {
        let keys = [0, 1, 2];
        assert_eq!(
            toposort_indices_with_keys([(0, 1), (1, 2)], &keys).unwrap(),
            [0, 1, 2]
        );
    }

    #[test]
    fn key_identity_matches_indices() {
        let keys = [0, 1, 2];
        let edges = [(0, 2), (1, 2)];
        let o = toposort_indices_with_keys(edges, &keys).unwrap();
        assert_eq!(o, [0, 1, 2]);
    }

    #[test]
    fn key_reverse_ord() {
        let keys = [30, 10, 20];
        let edges = [(1, 2), (2, 0)];
        let o = toposort_indices_with_keys(edges, &keys).unwrap();
        assert_eq!(o, [1, 2, 0]);
    }

    #[test]
    fn key_tuple_primary_secondary() {
        let keys = [(2, 1), (1, 2), (1, 1)];
        let edges = [(0, 1), (2, 1)];
        let o = toposort_indices_with_keys(edges, &keys).unwrap();
        let pos = |i| o.iter().position(|&x| x == i).unwrap();
        assert!(pos(2) < pos(1) && pos(0) < pos(1));
    }

    #[test]
    fn cycle_error() {
        let keys = ["x", "y"];
        let r = toposort_indices_with_keys([(0, 1), (1, 0)], &keys);
        assert!(r.is_err());
    }

    #[test]
    fn key_constant_preserves_topo() {
        let keys = [0, 0, 0];
        let edges = [(0, 2), (1, 2)];
        let o = toposort_indices_with_keys(edges, &keys).unwrap();
        assert_eq!(o.len(), 3);
        let pos = |i| o.iter().position(|&x| x == i).unwrap();
        assert!(pos(0) < pos(2) && pos(1) < pos(2));
    }

    #[test]
    fn key_large_dag() {
        let keys: Vec<i32> = (0..50).collect();
        let edges: Vec<(usize, usize)> = (0..49).map(|i| (i, i + 1)).collect();
        let o = toposort_indices_with_keys(edges, &keys).unwrap();
        assert_eq!(o, (0..50).collect::<Vec<_>>());
    }

    #[test]
    fn key_string_alphabetic() {
        let keys = ["M", "A", "Z"];
        let edges = [(1, 0), (1, 2)];
        let o = toposort_indices_with_keys(edges, &keys).unwrap();
        assert_eq!(o[0], 1);
        assert_eq!(o[2], 2);
    }
}
