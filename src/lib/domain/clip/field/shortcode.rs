use std::str::FromStr;

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::lib::domain::clip::ClipError;

#[derive(Debug, Clone, Serialize, Deserialize, From, Default)]
pub struct ShortCode(String);

impl ShortCode {
    #[must_use]
    pub fn new() -> Self {
        use rand::prelude::*;
        let allowed_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();

        let mut rng = thread_rng();
        let mut shortcode = String::with_capacity(10);

        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("Sampling array should have values"),
            );
        }
        Self(shortcode)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0
    }
}

impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        Self(shortcode.to_owned())
    }
}

impl FromStr for ShortCode {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}
