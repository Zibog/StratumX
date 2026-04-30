//! StratumX Editor Application Library
//!
//! This library exposes editor modules for testing and integration.
//! The editor app is a thin launcher - all business logic lives in
//! canonical crates in 5.editor, 4.tooling, 3.sdk, and 2.engine.

#![deny(unused_imports)]
#![deny(unused_variables)]
#![warn(dead_code)]

pub mod app_runtime;
#[cfg(feature = "desktop")]
pub mod desktop_app;
pub mod editor_host;
pub mod shell_bootstrap;
