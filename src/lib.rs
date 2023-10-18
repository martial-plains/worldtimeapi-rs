//! # World Time Wrapper
//!
//! This is a simple wrapper for the [World Time API](https://worldtimeapi.org/api/timezone/).
//!
//! ## Usage
//!
//! To use this crate, add `worldtimeapi` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! worldtimeapi = "0.4.1"
//! ```
//!
//! Then create a client for an endpoint (currently they only offer "ip" and "timezone"):
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! use worldtimeapi::service::{Client, Endpoint};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), reqwest::Error> {
//!     let client = Client::new(Endpoint::Timezone).await?;
//!
//!     let mut payload = HashMap::new();
//!     payload.insert("area", "America");
//!     payload.insert("location", "New_York");
//!
//!     let result = client.get(payload).await?;
//!     println!("{}", result.datetime());
//!     Ok(())
//! }
//! ```
//!
//! To get a list of regions and locations, use the `regions` method:
//!
//! ```rust
//! use worldtimeapi::service::{Client, Endpoint};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), reqwest::Error> {
//!     let client = Client::new(Endpoint::Timezone).await?;
//!     let regions = client.regions();
//!     println!("{:?}", regions);
//!     Ok(())  
//! }  
//! ```
//!

/// A collection of JSON responses from the [World Time API](https://worldtimeapi.org/api).
pub mod schema;

/// The client for the [World Time API](https://worldtimeapi.org/api).
pub mod service;
