use std::str::FromStr;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqlitePool, SqlitePoolOptions, SqliteQueryResult, SqliteRow},
    Pool, Sqlite,
};
use uuid::Uuid;

pub mod model;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::module_name_repetitions)]
pub enum DataError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = SqliteRow;
pub type AppQueryResult = SqliteQueryResult;

pub struct Database<D: sqlx::Database>(Pool<D>);

impl Database<Sqlite> {
    /// # Panics
    ///
    /// This function panics if the database cannot be connected to.
    pub async fn new(connection_str: &str) -> Self {
        let pool = SqlitePoolOptions::new().connect(connection_str).await;

        match pool {
            Ok(pool) => Self(pool),
            Err(err) => panic!(
                "Failed to connect to database: {}\n\
                \nIf the database has not yet been created, run:\n\
                \t$ sqlx database setup\n",
                err
            ),
        }
    }

    #[must_use]
    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

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
