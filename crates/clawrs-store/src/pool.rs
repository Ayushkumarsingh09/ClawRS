use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("{0}")]
    Message(String),
}

pub type StoreResult<T> = Result<T, StoreError>;

pub struct StorePool(Pool<Sqlite>);

impl StorePool {
    pub async fn connect(database_url: &str) -> StoreResult<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(16)
            .connect(database_url)
            .await?;
        Ok(Self(pool))
    }

    pub fn inner(&self) -> &Pool<Sqlite> {
        &self.0
    }

    pub async fn migrate(&self) -> StoreResult<()> {
        sqlx::migrate!("./migrations").run(self.inner()).await?;
        Ok(())
    }
}
