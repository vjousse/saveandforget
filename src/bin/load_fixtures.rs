extern crate dotenv;
extern crate saveandforget as saf;

use dotenv::dotenv;

use saf::db;
use saf::db::PgPool;

mod config {
    pub use config::ConfigError;
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct Config {
        pub database_url: String,
    }
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}

fn main() {
    dotenv().ok();

    let config = crate::config::Config::from_env().expect("Error reading .env file.");
    let pg_pool: PgPool = db::init_pool(&config.database_url).expect("Failed to create pool");
}
