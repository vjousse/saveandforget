use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_connection_pool() -> PgPool {
    dotenv().ok(); // This will load our .env file.

    // Load the DATABASE_URL env variable into database_url, in case of error
    // it will through a message "DATABASE_URL must be set"
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    init_pool(&database_url).expect("Failed to create pool")
}