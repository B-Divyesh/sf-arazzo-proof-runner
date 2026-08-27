//! A small, typed API for executing the supported Arazzo subset and comparing proofs.
//!
//! Most consumers should use the `arazzo-proof` binary. Embedders can call
//! [`run_workflow`] and [`compare_proofs`] without shelling out.

mod compare;
mod model;
mod parse;
mod redact;
mod report;
mod runner;

pub use compare::{compare_proofs, write_comparison};
pub use model::{Comparison, Environment, ProofBundle, RunStatus};
pub use runner::{RunOptions, run_workflow, write_proof};
