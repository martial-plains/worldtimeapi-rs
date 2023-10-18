use std::collections::HashMap;

use worldtimeapi::service::{Client, Endpoint};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new(Endpoint::Timezone).await?;

    let mut requests = HashMap::new();
    requests.insert("area", "America");
    requests.insert("location", "New_York");

    // Returns DateTimeJson object
    let response = client.get(requests).await?;

    println!("{:#?}", response);

    Ok(())
}
