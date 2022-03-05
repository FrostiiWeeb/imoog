use serde::Deserialize;
use std::{fs, io::Error};

#[derive(Deserialize)]
pub struct PostgresOptions {
    pub connection_uri: String,
    pub max_connections: u32,
}

impl PostgresOptions {
    fn from_config(&self) -> Result<Self, Error> {
        let config = fs::read_to_string("imoog.config.toml")?;
        let opts: Self = toml::from_str(config.as_str())?;

        Ok(opts)
    }
}

#[derive(Deserialize)]
pub struct MongoOptions {
    pub connection_uri: String,
    pub database_name: String,
    pub collection_name: String,
}

impl MongoOptions {
    fn from_config(&self) -> Result<Self, Error> {
        let config = fs::read_to_string("imoog.config.toml")?;
        let opts: Self = toml::from_str(config.as_str())?;

        Ok(opts)
    }
}
