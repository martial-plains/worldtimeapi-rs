use std::collections::HashMap;

use worldtimeapi::service::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("timezone").await;

    let mut requests = HashMap::new();
    requests.insert("area", "America");
    requests.insert("location", "New_York");

    // Returns DateTimeJson object
    let response = client.get(requests).await.unwrap();

    println!("{:#?}", response);
}
