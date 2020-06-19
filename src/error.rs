use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpelError {
    #[error("error executing DB query: {0}")]
    DBQueryError(#[from] tokio_postgres::Error),
}
