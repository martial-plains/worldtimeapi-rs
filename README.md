# World Time Wrapper

[![Rust](https://github.com/powpow58/worldtimeapi-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/powpow58/worldtimeapi-rs/actions/workflows/rust.yml)

This is a simple wrapper for the [World Time API](http://worldtimeapi.org). This crate is based on the [WorldTimeAPI wrapper](https://github.com/Dulatr/WorldTimeAPI) by Dulatr.

## Usage

To use this crate, add `worldtimeapi` to your `Cargo.toml`:

```toml
[dependencies]
worldtimeapi = "0.5.0"
```

Then create a client for an endpoint (currently they only offer "ip" and "timezone"):

```rust
use std::collections::HashMap;

use worldtimeapi::service::{Client, Endpoint};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
   let client = Client::new(Endpoint::Timezone).await?;

   let mut payload = HashMap::new();
   payload.insert("area", "America");
   payload.insert("location", "New_York");

   let result = client.get(payload).await?;
   println!("{}", result.datetime());
   Ok(())
}
```

To get a list of regions and locations, use the `regions` method:

```rust
use worldtimeapi::service::{Client, Endpoint};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
   let client = Client::new(Endpoint::Timezone).await?;
   let regions = client.regions();
   println!("{:?}", regions);
   Ok(())
}
```
