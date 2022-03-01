use async_trait::async_trait;
use crate::options::{
    MongoOptions,
    PostgresOptions
};
use sqlx::postgres::PgPoolOptions;


#[async_trait]
trait DatabaseImpl<OptionsT> {
    async fn connect(&self);
    async fn fetch(&self, identifier: String) -> (String, Vec<u8>, String);
    async fn insert(&self, identifier: String, mime_type: String, image: Vec<u8>);
    async fn delete();
    fn new(options: OptionsT) -> Self;
}

struct DatabaseDriver<OptionsT, ConnectionT> {
    options: OptionsT,
    connection: ConnectionT
}

// TODO: Implement all trait items

#[async_trait]
impl DatabaseImpl<PostgresOptions> for DatabaseDriver<PostgresOptions, sqlx::Pool<sqlx::Postgres>> {
    async fn connect(&self) {
        let conn = PgPoolOptions::new()
            .max_connections(self.options.max_connections)
            .connect(self.options.connection_uri.as_str())
            .await
            .unwrap();

        self.connection = conn;

        // execute the basic table initialization for imoog
        sqlx::query("CREATE TABLE IF NOT EXISTS imoog(
            image_identifier TEXT PRIMARY KEY,
            image_data BLOB,
            mime TEXT
        )")
            .execute(&conn)
            .await
            .expect("Failed to run the initialization query for imoog (postgres)");
    }

    async fn fetch(&self, identifier: String) -> (String, Vec<u8>, String) {
        /*
        Schema:
        image_identifier TEXT PRIMARY KEY,
        image_data BLOB
        mime TEXT
        */
        let row: (String, Vec<u8>, String) = sqlx::query_as("SELECT * FROM imoog WHERE image_identifier = $1")
            .bind(identifier)
            .fetch_one(&self.connection)
            .await
            .unwrap();

        row
    }

    async fn insert(&self, identifier: String, mime_type: String, image: Vec<u8>) {
        sqlx::query("INSERT INTO imoog VALUES($1, $2, $3)")
            .bind(identifier)
            .bind(image)
            .bind(mime_type)
            .execute(&self.connection)
            .await
            .expect(&format!("Failed to insert image ({})", identifier));
    }
}