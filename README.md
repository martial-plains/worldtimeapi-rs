# World Time Wrapper

This is a simple wrapper for the [World Time API](https://worldtimeapi.org/api/timezone/). This crate is based on the [WorldTimeAPI wrapper](https://github.com/Dulatr/WorldTimeAPI) by Dulatr.

## Usage

To use this crate, add `worldtimeapi` to your `Cargo.toml`:

```toml
[dependencies]
worldtimeapi = "0.1"
```

Then create a client for an endpoint (currently they only offer "ip" and "timezone"):

```rust
use std::collections::HashMap;
use worldtimeapi::service::Client;

#[tokio::main]
async fn main() {
   let client = Client::new("timezone").await;
   let payload = HashMap::new();
   
   payload.insert("area", "America");
   payload.insert("location", "New_York");
   
   let result = client.get(payload).await.unwrap();
   
   println!("{}", result.datetime());
}
```

To get a list of regions and locations, use the `regions` method:

```rust
let regions = client.regions().unwrap();
println!("{:?}", regions);
```
