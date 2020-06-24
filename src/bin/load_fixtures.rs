extern crate dotenv;
extern crate saveandforget as saf;
#[macro_use]
extern crate log;

use dotenv::dotenv;

use saf::db;
use saf::db::PgPool;
use saf::models::user::{RegisterUser, User};

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
    env_logger::init();
    debug!("Loading fixtures");

    let config = crate::config::Config::from_env().expect("Error reading .env file.");
    let pg_pool: PgPool = db::init_pool(&config.database_url).expect("Failed to create pool");

    let first_user = RegisterUser {
        email: "vin.cent@hey.com".to_owned(),
        password: "test".to_owned(),
        password_confirmation: "test".to_owned(),
    };
    let user_created = User::create(first_user, &pg_pool);
    debug!("{:?}", user_created);
}
