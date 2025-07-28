use elasticsearch::{Elasticsearch, http::transport::Transport};

use crate::config::FulltextSearchConfig;
pub mod user_fulltext_search;
pub fn init_fulltext_search(cfg: FulltextSearchConfig) -> Elasticsearch {
    let transport = Transport::single_node(cfg.urls.first().unwrap()).unwrap(); // TODO: make it generel
    Elasticsearch::new(transport)
}
