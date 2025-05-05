use async_trait::async_trait;
#[derive(Clone)]
pub struct HelloServiceImpl {}
impl HelloServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait]
pub trait HelloService: 'static + Sync + Send {
    async fn hello_world(&self) -> String;
}
#[async_trait]
impl HelloService for HelloServiceImpl {
    async fn hello_world(&self) -> String {
        "Hello, World".to_string()
    }
}
