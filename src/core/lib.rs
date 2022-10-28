// External
use diesel::{
    r2d2::{
        ConnectionManager,
        Pool,
    },
    sqlite::SqliteConnection
};
use failure::Error;

pub fn build_connection_pool(path: String) -> Result<Pool<ConnectionManager<SqliteConnection>>, Error>
{
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(
            ConnectionManager::<SqliteConnection>::new(path)
        )?;

    Ok(pool)
}