use dotenv::dotenv;
use redis::Client;
use std::env;

pub fn get_redis_client() -> Client {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    redis::Client::open(redis_url).expect("Failed to Connect")
}
