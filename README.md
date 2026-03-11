# stable_toposort

[![Crates.io](https://img.shields.io/crates/v/stable_toposort.svg)](https://crates.io/crates/stable_toposort)
[![Docs.rs](https://docs.rs/stable_toposort/badge.svg)](https://docs.rs/stable_toposort)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-APACHE)

Deterministic, stable topological sorting and related DAG algorithms for Rust.

## What it does

Given a DAG and an ordering of nodes, the crate computes a **topological order that minimizes inversions** with respect to that ordering. It also provides layered order (for parallel scheduling), strongly connected components (Tarjan), and condensation.

## Implemented

- stable topological sort (`toposort`, `toposort_by_key`)
- layered topological order (`toposort_layers`, `toposort_layers_by_key`)
- strongly connected components (`scc`, `scc_by_key`)
- condensation graph (`condensation`, `condensation_by_key`)
- toposort of SCCs (`toposort_scc`, `toposort_scc_by_key`)
- cycle detection (`CycleError<N>` with `.cycle`, `find_cycle`)

API is `nodes` + `edges` iterators; `(a, b)` means a → b. No graph type required.

## Example

```rust
use stable_toposort::toposort;

let order = toposort(["A", "B", "C"], [("A", "C"), ("B", "C")]).unwrap();
assert_eq!(order, ["A", "B", "C"]);
```

## License

MIT OR Apache-2.0
