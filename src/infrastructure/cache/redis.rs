use dotenv::dotenv;
use redis::cluster::ClusterClient;
use std::env;

pub fn get_redis_cluster_client() ->ClusterClient {
    dotenv().ok();
    let redis_urls = env::var("REDIS_URLS").expect("REDIS_URLS must be set");

    let nodes: Vec<String> = redis_urls
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    ClusterClient::new(nodes).unwrap()
}