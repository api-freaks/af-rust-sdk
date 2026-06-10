//! # APIFreaks APIs SDK
//!
//! The official Rust SDK for the APIFreaks APIs.
//!
//! ## Getting Started
//!
//! ```rust
//! use apifreaks::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         ..Default::default()
//!     };
//!     let client = ApiFreaks::new(config).expect("Failed to build client");
//!     client
//!         .geolocation_lookup(
//!             &GeolocationLookupQueryRequest {
//!                 api_key: "apiKey".to_string(),
//!                 format: None,
//!                 ip: None,
//!                 lang: None,
//!                 fields: None,
//!                 excludes: None,
//!                 include: None,
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
