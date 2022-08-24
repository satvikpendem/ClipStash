use std::str::FromStr;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default, From, Display)]
pub struct DbId(Uuid);

impl DbId {
    #[must_use]
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }

    #[must_use]
    pub fn nil() -> Self {
        Self(Uuid::nil())
    }

    #[must_use]
    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::from_str(raw)?))
    }
}
