use derive_more::{Constructor, From};
use serde::{Deserialize, Serialize};

use crate::lib::data::DbId;

#[derive(Debug, Clone, Serialize, Deserialize, Constructor, From)]
pub struct ClipId(DbId);

impl ClipId {
    #[must_use]
    pub fn into_inner(self) -> DbId {
        self.0
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}
