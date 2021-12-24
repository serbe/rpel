use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::{NoTls, config::Host};

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

fn option_str(value: Option<&str>) -> Option<String> {
    value.map(|v| v.to_owned())
}

fn get_config(pg_cfg: &str) -> Result<Config, RpelError> {
    let tokio_cfg = pg_cfg.parse::<tokio_postgres::Config>()?;
    let mut cfg = Config::new();
    cfg.user = option_str(tokio_cfg.get_user());
    cfg.password = tokio_cfg
        .get_password()
        .map(String::from_utf8_lossy)
        .map(|cow| cow.to_string());
    cfg.dbname = option_str(tokio_cfg.get_dbname());
    cfg.options = option_str(tokio_cfg.get_options());
    cfg.application_name = option_str(tokio_cfg.get_application_name());
    // cfg.ssl_mode = tokio_cfg.get_ssl_mode();
    cfg.host = tokio_cfg
        .get_hosts()
        .first()
        .map(|host| match host {
            Host::Tcp(host) => host.to_string(),
            #[cfg(unix)]
            Host::Unix(buf) => buf.to_string_lossy().to_string(),
        });
    cfg.port = tokio_cfg.get_ports().first().cloned();
    Ok(cfg)
}

pub fn get_pool(pg_cfg: &str) -> Result<RpelPool, RpelError> {
    let cfg = get_config(pg_cfg)?;
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
