use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use crate::lib::domain::time::Time;

#[derive(Debug, Clone, Serialize, Deserialize, Constructor)]
pub struct Posted(Time);

impl Posted {
    #[must_use]
    pub fn into_inner(self) -> Time {
        self.0
    }
}
