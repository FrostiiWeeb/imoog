use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PostgresOptions {
    pub connection_uri: String,
    pub max_connections: u32,
}

#[derive(Deserialize, Debug)]
pub struct MongoOptions {
    pub connection_uri: String,
    pub database_name: String,
    pub collection_name: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthOptions {
    secret_key: String,
    delete_requires_auth: bool,
}

#[derive(Deserialize, Debug)]
pub struct ImoogOptions {
    pub database: MongoOptions,
    pub auth: AuthOptions,
}
