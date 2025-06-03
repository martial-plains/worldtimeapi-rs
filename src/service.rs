use std::{collections::HashMap, fmt::Write, panic, string::ToString};

use derive_more::derive::Display;
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
#[derive(Debug, Display, Clone, Copy)]
pub enum Endpoint {
    /// The "timezone" endpoint.
    #[display("timezone")]
    Timezone,
    /// The "ip" endpoint.
    #[display("ip")]
    Ip,
}

impl Client {
    /// Create a new client for the specified endpoint.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The request to the World Time API fails (e.g., network issues).
    /// - The response cannot be deserialized into a JSON value.
    pub async fn new(endpoint: Endpoint) -> Result<Self, reqwest::Error> {
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

        let url: String = format!("https://worldtimeapi.org/api/{endpoint}");

        Ok(Self { regions, url })
    }

    /// Get the current time for the specified region.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The request to the World Time API fails.
    /// - The response cannot be deserialized into a `DateTimeJson`.
    /// - The `payload` is missing required keys like `area`, or contains invalid keys, in which case it panics.
    ///
    /// # Panics
    ///
    /// This function will panic if:
    /// - The `payload` contains invalid keys not among "area", "location", or "region".
    /// - The required key `area` is missing.
    /// - The `region` key is provided without a `location`.
    pub async fn get(&self, payload: HashMap<&str, &str>) -> Result<DateTimeJson, reqwest::Error> {
        let keys = payload
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        let mut args = String::new();

        for item in keys.clone() {
            if !["area", "location", "region"]
                .iter()
                .map(ToString::to_string)
                .any(|x| x == *item)
            {
                panic!("Invalid key: {item}");
            }
        }

        if keys.contains(&"area".to_string()) {
            write!(args, "/{}", payload["area"]).unwrap();
        } else {
            panic!("Missing key: area");
        }

        if keys.contains(&"location".to_string()) {
            write!(args, "/{}", payload["location"]).unwrap();
        }

        if keys.contains(&"location".to_string()) && keys.contains(&"region".to_string()) {
            write!(args, "/{}", payload["region"]).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[tokio::test]
    async fn test_client_get_success() {
        let mut server = mockito::Server::new_async().await;
        let mock_response = r#"
        {
            "abbreviation": "EDT",
            "client_ip": "192.0.2.1",
            "datetime": "2024-09-23T14:23:48+00:00",
            "day_of_week": 1,
            "day_of_year": 266,
            "dst": true,
            "dst_from": null,
            "dst_offset": 3600,
            "dst_until": null,
            "raw_offset": -18000,
            "timezone": "America/New_York",
            "unixtime": 1695475428,
            "utc_datetime": "2024-09-23T14:23:48Z",
            "utc_offset": "-05:00",
            "week_number": 39
        }
        "#;

        // Mocking the World Time API for a timezone query
        let _m = server
            .mock("GET", "/api/timezone/America/New_York")
            .with_status(200)
            .with_body(mock_response)
            .create();

        let client = Client {
            regions: vec![],
            url: format!("{}/api/timezone", server.url()),
        };

        let mut payload = HashMap::new();
        payload.insert("area", "America");
        payload.insert("location", "New_York");

        let response = client.get(payload).await.unwrap();

        // Verifying that the response matches the mock response
        let expected = serde_json::from_str::<DateTimeJson>(mock_response).unwrap();
        assert_eq!(response.abbreviation(), expected.abbreviation());
        assert_eq!(response.client_ip(), expected.client_ip());
    }

    #[tokio::test]
    #[should_panic(expected = "Invalid key: invalid_key")]
    async fn test_client_get_invalid_key() {
        let server = mockito::Server::new_async().await;
        let client = Client {
            regions: vec![],
            url: format!("{}/api/timezone", server.url()),
        };

        let mut payload = HashMap::new();
        payload.insert("invalid_key", "America");

        // This should panic because "invalid_key" is not a valid key
        client.get(payload).await.unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "Missing key: area")]
    async fn test_client_get_missing_area() {
        let server = mockito::Server::new_async().await;
        let client = Client {
            regions: vec![],
            url: format!("{}/api/timezone", server.url()),
        };

        let mut payload = HashMap::new();
        payload.insert("location", "New_York");

        // This should panic because the "area" key is missing
        client.get(payload).await.unwrap();
    }

    #[test]
    fn test_endpoint_display() {
        assert_eq!(Endpoint::Timezone.to_string(), "timezone");
        assert_eq!(Endpoint::Ip.to_string(), "ip");
    }
}
