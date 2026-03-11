//! Deterministic and stable topological sorting algorithms for DAGs.
//!
//! This crate provides topological sort that minimizes inversions relative to
//! a given node ordering.

mod toposort;
mod layers;
mod scc;
mod condensation;

pub use toposort::{stable_toposort, stable_toposort_by_key};
pub use layers::{toposort_layers, toposort_layers_by_key};
pub use scc::{scc, scc_by_key};
pub use condensation::{Condensation, condensation, condensation_by_key, stable_toposort_scc, stable_toposort_scc_by_key};

/// Error returned when the graph contains a cycle (is not a DAG).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleError;
