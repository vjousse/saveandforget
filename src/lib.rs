#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

extern crate reqwest;

pub mod core;
pub mod db;
pub mod errors;
pub mod messenger;
pub mod models;
pub mod schema;
pub mod web_handlers;
