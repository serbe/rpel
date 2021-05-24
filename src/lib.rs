use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};

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

pub type RpelPool = Pool<NoTls>;

fn get_config(pg_cfg: &str) -> Result<Config, RpelError> {
    let config = pg_cfg.parse()?;
    Ok(config)
}

pub fn get_pool(pg_cfg: &str) -> Result<RpelPool, RpelError> {
    let pg_config = get_config(pg_cfg)?;
    let manager = Manager::new(pg_config, NoTls);
    let pool = Pool::new(manager, 16);
    Ok(pool)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
