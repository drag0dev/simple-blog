use diesel::{r2d2::ConnectionManager, PgConnection};
use anyhow::{Context, Result};

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool(db_url: String) -> Result<DBPool> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .context("creating r2d2 pool")
}
