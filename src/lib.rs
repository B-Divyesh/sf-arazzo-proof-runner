//! A small, typed API for executing the supported Arazzo subset and comparing proofs.
//!
//! Most consumers should use the `arazzo-proof` binary. Embedders can call
//! [`run_workflow`] and [`compare_proofs`] without shelling out.
//!
//! ```no_run
//! use arazzo_proof_runner::{RunOptions, RunStatus, run_workflow};
//! use std::path::PathBuf;
//!
//! let proof = run_workflow(&RunOptions {
//!     arazzo_path: PathBuf::from("workflow.arazzo.yaml"),
//!     environment_path: PathBuf::from("ci.env.yaml"),
//!     workflow_id: Some("checkout".to_owned()),
//! })?;
//! assert_eq!(proof.result, RunStatus::Passed);
//! # Ok::<(), anyhow::Error>(())
//! ```

mod compare;
mod model;
mod parse;
mod redact;
mod report;
mod runner;

pub use compare::{compare_proofs, write_comparison};
pub use model::{Comparison, Environment, ProofBundle, RunStatus};
pub use runner::{RunOptions, run_workflow, write_proof};
