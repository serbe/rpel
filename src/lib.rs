#![allow(dead_code)]
use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};

mod certificate;
mod company;
mod contact;
mod department;
mod education;
mod email;
mod kind;
mod phone;
mod post;
mod practice;
mod rank;
mod scope;
mod select;
mod siren;
mod siren_type;
mod tcc;

fn get_config() -> Config {
    let mut config = Config::new();
    if let Ok(dbname) = dotenv::var("DB_NAME") {
        config.dbname(&dbname);
    };
    if let Ok(user) = dotenv::var("DB_USER") {
        config.user(&user);
    };
    if let Ok(password) = dotenv::var("DB_PASSWORD") {
        config.password(&password);
    };
    if let Ok(host) = dotenv::var("DB_HOST") {
        config.host(&host);
    };
    if let Ok(port) = dotenv::var("DB_PORT") {
        config.port(port.parse().expect("port need u16 type"));
    };
    config
}

pub fn get_pool() -> Pool {
    dotenv::dotenv().ok();
    let manager = Manager::new(get_config(), NoTls);
    Pool::new(manager, 16)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
