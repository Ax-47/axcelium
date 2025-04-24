use async_trait::async_trait;

pub struct HelloRepositoryImpl {}
impl Default for HelloRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}
impl HelloRepositoryImpl {
    pub fn new() -> Self {
        HelloRepositoryImpl {}
    }
}

#[async_trait]
pub trait HelloRepository: Send + Sync {
    async fn hello_world(&self) -> String;
}
#[async_trait]
impl HelloRepository for HelloRepositoryImpl {
    async fn hello_world(&self) -> String {
        "Hello, World".to_string()
    }
}
