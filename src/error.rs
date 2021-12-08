use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpelError {
    #[error("executing DB query: {0}")]
    DBQuery(#[from] tokio_postgres::Error),
    #[error("config: {0}")]
    Config(#[from] config::ConfigError),
    #[error("create pool: {0}")]
    CreatePool(#[from] deadpool_postgres::CreatePoolError),
    #[error("pool: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),
}
