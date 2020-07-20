use deadpool_postgres::{Manager, Pool};
use tokio_postgres::{Config, NoTls};

pub mod error;

pub mod certificate;
pub mod company;
pub mod contact;
pub mod department;
pub mod education;
pub mod email;
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
