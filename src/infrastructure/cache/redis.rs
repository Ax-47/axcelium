use redis::cluster::ClusterClient;
use crate::config;

pub fn get_redis_cluster_client(cfg: config::RedisConfig) ->ClusterClient {

    ClusterClient::new(cfg.urls).unwrap()
}