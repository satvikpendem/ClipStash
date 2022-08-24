use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Constructor, Serialize, Deserialize)]
pub struct Hits(u64);

impl Hits {
    #[must_use]
    pub const fn into_inner(self) -> u64 {
        self.0
    }
}
