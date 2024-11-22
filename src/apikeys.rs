use diesel::pg::Pg;
use diesel::prelude::Insertable;
use diesel::prelude::Queryable;
use diesel::prelude::QueryableByName;
use diesel::Selectable;

use crate::schema::{self, apikeys};

#[derive(Insertable)]
#[derive(Queryable)]
#[derive(Selectable)]
#[table_name = "apikeys"]
#[derive(serde::Deserialize)]
#[diesel(check_for_backend(Pg))]
#[derive(QueryableByName)]
pub struct ApiKey {
    pub api_key: String,
    pub wallet_address: String,
    pub transaction_limit: i32,
    pub whitelisted_addresses: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct  ApiKeyWhitelist {
    pub api_key: String,
    pub whitelist_addresses: String,
}