use std::path::PathBuf;

use crate::db_connection::PgPooledConnection;

pub struct AppState {
    pub fb_verify_token: String,
    pub download_path: PathBuf,
    pub pg_pool: PgPooledConnection,
}
