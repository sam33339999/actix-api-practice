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

// 建立連線: 未使用連接池，直接進行連接
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// 建立連接池，使用 diesel r2d2 pool. (用於下面 lazy_static! 進行 Arc 初始化建立連接池)
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

// lazy static 進行共享池設定
lazy_static! {
    static ref CONNECTION_POOL: Arc<Pool<ConnectionManager<PgConnection>>> =
        Arc::new(create_connection_pool());
}

// 取得共享池，用共享池資源進行 db 連接(復用)
pub fn get_connection_pool() -> Arc<Pool<ConnectionManager<PgConnection>>> {
    // 這邊可以拿到連接池的狀態， 連線數與閒置連線數。
    // let state = CONNECTION_POOL.state();
    // println!("pool state: {:#?}", state);
    CONNECTION_POOL.clone()
}
