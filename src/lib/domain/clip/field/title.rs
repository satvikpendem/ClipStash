use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::lib::domain::clip::ClipError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();

        if let Some(title) = title {
            if !title.trim().is_empty() {
                return Self(Some(title));
            }
        }
        Self(None)
    }

    #[must_use]
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}
