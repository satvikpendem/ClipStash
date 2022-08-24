use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod field;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub clip_id: field::clip_id::ClipId,
    pub shortcode: field::shortcode::ShortCode,
    pub content: field::content::Content,
    pub title: field::title::Title,
    pub posted: field::posted::Posted,
    pub expires: field::expires::Expires,
    pub password: field::password::Password,
    pub hits: field::hits::Hits,
}

#[derive(Debug, Error)]
#[allow(clippy::module_name_repetitions)]
pub enum ClipError {
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    #[error("Invalid title: {0}")]
    InvalidTitle(String),
    #[error("Empty content")]
    EmptyContent,
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    #[error("Date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("ID parse error: {0}")]
    Id(#[from] uuid::Error),
    #[error("Hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}
