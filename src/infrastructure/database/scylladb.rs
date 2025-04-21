use dotenv::dotenv;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::env;

pub async fn get_db_pool() -> Session {
    dotenv().ok();
    let database_urls = env::var("DATABASE_URLS").expect("DATABASE_URL must be set")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    let database_username = env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set");
    let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let mut builder = SessionBuilder::new();
    for url in database_urls {
        builder = builder.known_node(url);
    }
    let session = builder
        .user(database_username, database_password)
        .build()
        .await.unwrap();
    session
}
