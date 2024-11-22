use diesel::prelude::*;
use dotenv::dotenv;
use::std::env;

pub async fn connect_db() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&database_url).unwrap()
}