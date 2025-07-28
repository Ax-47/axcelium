use thiserror::Error;
#[derive(Debug, Error)]
pub enum FulltextSearchError {
    #[error("Elasticsearch error: {0}")]
    ElasticError(#[from] elasticsearch::Error),

    #[error("Elasticsearch indexing failed with status: {0}")]
    IndexingFailed(String),
}
