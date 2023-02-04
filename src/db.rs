use dotenv::dotenv;
pub use mongodb::bson::{doc, Document};
pub use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Database,
};
use std::env;

pub async fn connect_database() -> Client {
    dotenv().ok();
    let client_uri: String = env::var("MONGODB_URI").unwrap();
    //  println!(" connecting to the database with URL: {}", client_uri);

    // 1. connecting to the database
    let mut _client_options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .expect("failed to connect to the database!!!");

    // 2. Getting handle to the database
    let client = Client::with_options(_client_options).expect("failed to handle the database");

    return client;
}
