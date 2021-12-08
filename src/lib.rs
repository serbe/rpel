use deadpool_postgres::{Pool, Runtime};
use dotenv::dotenv;
use serde::Deserialize;
use tokio_postgres::NoTls;

use crate::error::RpelError;

pub mod certificate;
pub mod company;
pub mod contact;
pub mod department;
pub mod education;
pub mod email;
pub mod error;
pub mod kind;
pub mod phone;
pub mod post;
pub mod practice;
pub mod rank;
pub mod scope;
pub mod select;
pub mod siren;
pub mod siren_type;
pub mod tcc;
pub mod user;

pub type RpelPool = Pool;

#[derive(Debug, Deserialize)]
struct Config {
    pg: deadpool_postgres::Config,
}

impl Config {
    fn from_env() -> Result<Self, RpelError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new().separator("__"))?;
        Ok(cfg.try_into()?)
    }
}

pub fn get_pool() -> Result<RpelPool, RpelError> {
    dotenv().ok();
    let cfg = Config::from_env()?;
    let pool = cfg.pg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
