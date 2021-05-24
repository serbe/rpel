use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpelError {
    #[error("executing DB query: {0}")]
    DBQuery(#[from] tokio_postgres::Error),
    #[error("config: {0}")]
    Config(#[from] deadpool_postgres::config::ConfigError),
    #[error("pool: {0}")]
    Pool(#[from] deadpool_postgres::PoolError),
}
