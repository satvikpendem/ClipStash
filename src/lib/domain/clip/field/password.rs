use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::lib::domain::clip::ClipError;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct Password(Option<String>);

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();

        if let Some(password) = password {
            if !password.trim().is_empty() {
                return Ok(Self(Some(password)));
            }
        }
        Ok(Self(None))
    }

    #[must_use]
    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    #[must_use]
    pub const fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}
