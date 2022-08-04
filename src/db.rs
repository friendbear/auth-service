
use super::vars;

use diesel::{r2d2::ConnectionManager, PgConnection};

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> Pool {
    // create a database connection pool
    let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Faild to create a database connection pool.");
    pool.clone()
}