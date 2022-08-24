use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, From)]
pub struct Time(DateTime<Utc>);

impl Time {
    #[must_use]
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    #[must_use]
    pub fn timestamp(self) -> i64 {
        self.0.timestamp()
    }

    #[must_use]
    pub fn from_naive_utc(datetime: NaiveDateTime) -> Self {
        Self(DateTime::from_utc(datetime, Utc))
    }
}

impl FromStr for Time {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(format!("{}T00:00:00Z", s).parse::<DateTime<Utc>>()?.into())
    }
}
