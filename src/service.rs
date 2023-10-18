use std::{collections::HashMap, fmt::Display, panic};

use serde_json::Value;

use crate::schema::DateTimeJson;

/// Create a World Time API client for specified endpoint.
///
/// Support endpoints are "timezone" and "ip".
#[derive(Debug)]
pub struct Client {
    regions: Vec<Value>,
    url: String,
}

/// An enumeration of the supported endpoints for the World Time API client.
///
/// The World Time API provides two endpoints: "timezone" and "ip".
/// The "timezone" endpoint allows querying the current time for a specific timezone region.
/// The "ip" endpoint allows querying the current time for a specific IP address.
#[derive(Debug, Clone, Copy)]
pub enum Endpoint {
    /// The "timezone" endpoint.
    Timezone,
    /// The "ip" endpoint.
    Ip,
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Endpoint::Timezone => "timezone",
                Endpoint::Ip => "ip",
            }
        )
    }
}

impl Client {
    /// Create a new client for the specified endpoint.
    pub async fn new(endpoint: Endpoint) -> Result<Client, reqwest::Error> {
        // for the timezone endpoint, define a region list property
        let regions: Vec<Value> = match endpoint {
            Endpoint::Timezone => {
                let url = "https://worldtimeapi.org/api/timezone/".to_string();
                let response = reqwest::get(&url).await?;

                response.json().await?
            }
            Endpoint::Ip => {
                let url = "https://worldtimeapi.org/api/ip".to_string();
                let response = reqwest::get(&url).await?;

                response.json().await?
            }
        };

        let url: String = format!("https://worldtimeapi.org/api/{}", endpoint);

        Ok(Client { regions, url })
    }

    /// Get the current time for the specified region.
    pub async fn get(&self, payload: HashMap<&str, &str>) -> Result<DateTimeJson, reqwest::Error> {
        let keys = payload
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut args = String::new();

        for item in keys.clone() {
            if !["area", "location", "region"]
                .iter()
                .map(|s| s.to_string())
                .any(|x| x == *item)
            {
                panic!("Invalid key: {}", item);
            }
        }

        if keys.contains(&"area".to_string()) {
            args.push_str(&format!("/{}", payload["area"]));
        } else {
            panic!("Missing key: area");
        }

        if keys.contains(&"location".to_string()) {
            args.push_str(&format!("/{}", payload["location"]));
        }

        if keys.contains(&"location".to_string()) && keys.contains(&"region".to_string()) {
            args.push_str(&format!("/{}", payload["region"]));
        } else if !keys.contains(&"location".to_string()) && keys.contains(&"region".to_string()) {
            panic!("Missing key: region");
        }

        let response = reqwest::get(&format!("{}{}", self.url, args))
            .await?
            .json::<DateTimeJson>()
            .await?;
        Ok(response)
    }

    /// Get a reference to the client's regions.
    #[must_use]
    pub fn regions(&self) -> &[Value] {
        self.regions.as_ref()
    }
}
