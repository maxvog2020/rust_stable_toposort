//! Deterministic and stable topological sorting for directed graphs.
//!
//! This crate provides algorithms that produce a consistent ordering for a given
//! graph and node order: same input always yields the same output. This is useful
//! for build systems, dependency resolution, and any application where
//! reproducible order matters.
//!
//! # Algorithms
//!
//! - **Topological sort**: [`toposort`] / [`toposort_by_key`] — order
//!   all nodes so that every edge goes from an earlier to a later node. Fails with
//!   [`CycleError`] if the graph has a cycle.
//! - **Layers**: [`toposort_layers`] / [`toposort_layers_by_key`] — group nodes into
//!   layers (e.g. for parallel execution); nodes in the same layer have no dependencies
//!   on each other.
//! - **Strongly connected components (SCC)**: [`scc`] / [`scc_by_key`] — partition
//!   the graph into maximal strongly connected components.
//! - **Condensation**: [`condensation`] / [`condensation_by_key`] — build the DAG
//!   of SCCs; [`Condensation`] holds the components and edges between component indices.
//! - **Toposort of SCCs**: [`toposort_scc`] / [`toposort_scc_by_key`] —
//!   return SCCs in topological order (each SCC as a `Vec<N>`).
//!
//! # Examples
//!
//! Topological sort (DAG):
//!
//! ```rust
//! use stable_toposort::toposort;
//!
//! let nodes = ["prepare", "compile", "link"];
//! let edges = [("prepare", "compile"), ("compile", "link")];
//! let order = toposort(nodes, edges).unwrap();
//! assert_eq!(order, ["prepare", "compile", "link"]);
//! ```
//!
//! Cycle detection:
//!
//! ```rust
//! use stable_toposort::{toposort, CycleError};
//!
//! let nodes = ["a", "b"];
//! let edges = [("a", "b"), ("b", "a")];
//! let err: CycleError<&str> = toposort(nodes, edges).unwrap_err();
//! assert_eq!(err.cycle, ["a", "b", "a"]);
//! ```
//!
//! Layers (for parallelization):
//!
//! ```rust
//! use stable_toposort::toposort_layers;
//!
//! let nodes = ["a", "b", "c"];
//! let edges = [("a", "c"), ("b", "c")];
//! let layers = toposort_layers(nodes, edges).unwrap();
//! assert_eq!(layers, vec![vec!["a", "b"], vec!["c"]]);
//! ```

mod cycle;
mod toposort;
mod layers;
mod scc;
mod condensation;

pub use cycle::{find_cycle, CycleError};
pub use toposort::{toposort, toposort_by_key};
pub use layers::{toposort_layers, toposort_layers_by_key};
pub use scc::{scc, scc_by_key};
pub use condensation::{Condensation, condensation, condensation_by_key, toposort_scc, toposort_scc_by_key};
