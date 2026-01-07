//! Cargo subcommand to forward commands to the nightly toolchain.
//!
//! This extension enables nightly toolchain usage in cargo aliases by
//! providing a `cargo-nightly` subcommand that forwards all arguments to
//! `cargo +nightly <args>`.
//!
//! ## Why it exists
//!
//! Cargo aliases in `.cargo/config.toml` cannot directly use `+nightly`
//! syntax because cargo aliases can only invoke cargo subcommands, not
//! external commands or toolchain selectors. This plugin provides a
//! `cargo-nightly` subcommand that forwards all arguments to
//! `cargo +nightly <args>`, making it possible to use nightly toolchain
//! commands through cargo aliases.
//!
//! ## Usage
//!
//! Once installed, you can use it directly:
//!
//! ```bash
//! cargo nightly clippy
//! cargo nightly check
//! ```
//!
//! Or through aliases in `.cargo/config.toml`:
//!
//! ```toml
//! [alias]
//! clippy2 = "nightly clippy"
//! check2 = "nightly clippy"
//! ```
//!
//! ## Example
//!
//! ```toml
//! # .cargo/config.toml
//! [alias]
//! clippy2 = "nightly clippy"
//! check2 = "nightly clippy"
//! clippy-all = "nightly clippy --workspace --all-targets --all-features -- -D warnings"
//! ```
//!
//! Then use the aliases:
//!
//! ```bash
//! cargo clippy2
//! cargo check2
//! cargo clippy-all
//! ```
