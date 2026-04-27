//! Project executor — project/build/release contour.
//!
//! Handles: bootstrap, save, build, export, launch, verify.

pub mod executor_project;

pub use executor_project::execute;
