use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::sync::Arc;

pub mod models;
pub mod posts;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .min_idle(Some(10))
        .max_size(30)
        .build(manager)
        .expect("Failed to create pool")
}

lazy_static! {
    static ref CONNECTION_POOL: Arc<Pool<ConnectionManager<PgConnection>>> =
        Arc::new(create_connection_pool());
}

pub fn get_connection_pool() -> Arc<Pool<ConnectionManager<PgConnection>>> {
    CONNECTION_POOL.clone()
}
