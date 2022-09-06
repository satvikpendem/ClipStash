use std::str::FromStr;

use chrono::NaiveDateTime;

use crate::{
    lib::domain::clip::field::{
        clip_id::ClipId, content::Content, expires::Expires, hits::Hits, password::Password,
        posted::Posted, shortcode::ShortCode, title::Title,
    },
    ClipError, Time,
};

use super::DbId;

#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::lib::data) clip_id: String,
    pub(in crate::lib::data) content: String,
    pub(in crate::lib::data) expires: Option<NaiveDateTime>,
    pub(in crate::lib::data) hits: i64,
    pub(in crate::lib::data) password: Option<String>,
    pub(in crate::lib::data) posted: NaiveDateTime,
    pub(in crate::lib::data) shortcode: String,
    pub(in crate::lib::data) title: String,
}

impl TryFrom<Clip> for crate::lib::domain::clip::Clip {
    type Error = ClipError;
    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        Ok(Self {
            clip_id: ClipId::new(DbId::from_str(clip.clip_id.as_str())?),
            content: Content::new(clip.content)?,
            expires: Expires::new(clip.expires.map(Time::from_naive_utc)),
            shortcode: ShortCode::from(clip.shortcode),
            title: Title::new(clip.title),
            posted: Posted::new(Time::from_naive_utc(clip.posted)),
            password: Password::new(clip.password.unwrap_or_default())?,
            hits: Hits::new(u64::try_from(clip.hits).unwrap()),
        })
    }
}

pub struct GetClip {
    pub(in crate::lib::data) shortcode: String,
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self {
            shortcode: shortcode.into_inner(),
        }
    }
}

impl From<String> for GetClip {
    fn from(shortcode: String) -> Self {
        Self { shortcode }
    }
}

pub struct NewClip {
    pub(in crate::lib::data) clip_id: String,
    pub(in crate::lib::data) content: String,
    pub(in crate::lib::data) expires: Option<i64>,
    pub(in crate::lib::data) hits: i64,
    pub(in crate::lib::data) password: Option<String>,
    pub(in crate::lib::data) posted: i64,
    pub(in crate::lib::data) shortcode: String,
    pub(in crate::lib::data) title: String,
}

pub struct UpdateClip {
    pub(in crate::lib::data) content: String,
    pub(in crate::lib::data) expires: Option<i64>,
    pub(in crate::lib::data) hits: i64,
    pub(in crate::lib::data) password: Option<String>,
    pub(in crate::lib::data) shortcode: String,
    pub(in crate::lib::data) title: String,
}
