
use rocket_db_pools::{Database, sqlx};


#[derive(Database)]
#[database("maindb")]
pub(crate) struct Users(sqlx::MySqlPool);