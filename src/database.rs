use sqlx::{Error, MySql, Pool};

pub struct Database {
    pub conn: Pool<MySql>
}

impl Database {
/** create a new database connection
    * requires [username] and [password]
    * returns [DatabaseService] self with the connection to the database
    * returns [err] the error while connecting to the database
 */
pub async fn new(database_url: &str) -> Result<Self, Error> {
    Ok(Self {
        conn: sqlx::mysql::MySqlPoolOptions::new()
            .connect(database_url)
            .await?,
    })
}
}