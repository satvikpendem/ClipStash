use serde::{Deserialize, Serialize};

use crate::lib::domain::clip::ClipError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: String) -> Result<Self, ClipError> {
        if content.trim().is_empty() {
            return Err(ClipError::EmptyContent);
        }
        Ok(Self(content))
    }

    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
