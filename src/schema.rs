use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Any valid JSON response form acquiring a `DateTime` from timezone endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct DateTimeJson {
    abbreviation: String,
    client_ip: String,
    datetime: String,
    day_of_week: i64,
    day_of_year: i64,
    dst: bool,
    dst_from: Option<DateTime<Utc>>,
    dst_offset: i64,
    dst_until: Option<DateTime<Utc>>,
    raw_offset: i64,
    timezone: String,
    unixtime: i64,
    utc_datetime: DateTime<Utc>,
    utc_offset: String,
    week_number: i64,
}

impl DateTimeJson {
    /// Returns the abbreviation of the timezone
    #[must_use]
    pub fn key_list() -> Vec<&'static str> {
        vec![
            "abbreviation",
            "client_ip",
            "datetime",
            "day_of_week",
            "day_of_year",
            "dst",
            "dst_from",
            "dst_offset",
            "dst_until",
            "raw_offset",
            "timezone",
            "unixtime",
            "utc_datetime",
            "utc_offset",
            "week_number",
        ]
    }

    /// Returns the abbreviation of the timezone
    #[must_use]
    pub fn abbreviation(&self) -> &str {
        &self.abbreviation
    }

    /// Returns the `client_ip` of the timezone
    #[must_use]
    pub fn client_ip(&self) -> &str {
        &self.client_ip
    }

    /// Returns the datetime of the timezone
    #[must_use]
    pub fn datetime(&self) -> &str {
        &self.datetime
    }

    /// Returns the `day_of_week` of the timezone
    #[must_use]
    pub const fn day_of_week(&self) -> i64 {
        self.day_of_week
    }

    /// Returns the `day_of_year` of the timezone
    #[must_use]
    pub const fn day_of_year(&self) -> i64 {
        self.day_of_year
    }

    /// Returns the dst of the timezone
    #[must_use]
    pub const fn dst(&self) -> bool {
        self.dst
    }

    /// Returns the `dst_from` of the timezone
    #[must_use]
    pub const fn dst_from(&self) -> Option<DateTime<Utc>> {
        self.dst_from
    }

    /// Returns the `dst_offset` of the timezone
    #[must_use]
    pub const fn dst_offset(&self) -> i64 {
        self.dst_offset
    }

    /// Returns the `dst_until` of the timezone
    #[must_use]
    pub const fn dst_until(&self) -> Option<DateTime<Utc>> {
        self.dst_until
    }

    /// Returns the `raw_offset` of the timezone
    #[must_use]
    pub const fn raw_offset(&self) -> i64 {
        self.raw_offset
    }

    /// Returns the timezone of the timezone
    #[must_use]
    pub fn timezone(&self) -> &str {
        &self.timezone
    }

    /// Returns the unixtime of the timezone
    #[must_use]
    pub const fn unixtime(&self) -> i64 {
        self.unixtime
    }

    /// Returns the `utc_datetime` of the timezone
    #[must_use]
    pub const fn utc_datetime(&self) -> DateTime<Utc> {
        self.utc_datetime
    }

    /// Returns the `utc_offset` of the timezone
    #[must_use]
    pub fn utc_offset(&self) -> &str {
        &self.utc_offset
    }

    /// Returns the `week_number` of the timezone
    #[must_use]
    pub const fn week_number(&self) -> i64 {
        self.week_number
    }
}
