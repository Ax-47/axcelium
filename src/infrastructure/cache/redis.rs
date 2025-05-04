use crate::config;
use redis::Client;

pub fn get_redis_client(cfg: config::RedisConfig) -> Client {
    Client::open(cfg.urls[0].as_str()).unwrap()
}