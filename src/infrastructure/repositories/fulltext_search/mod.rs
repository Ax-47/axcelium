use elasticsearch::{Elasticsearch, http::transport::Transport};
pub mod user_fulltext_search;
pub fn init_fulltext_search() -> Elasticsearch {
    let transport = Transport::single_node("https://example.com").unwrap();
    Elasticsearch::new(transport)
}
