use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::lib::domain::{clip::ClipError, time::Time};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<T: Into<Option<Time>>>(expires: T) -> Self {
        Self(expires.into())
    }

    #[must_use]
    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

impl FromStr for Expires {
    type Err = ClipError;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.is_empty() {
            return Ok(Self(None));
        }
        Ok(Self::new(Time::from_str(raw)?))
    }
}
