use serde::Serialize;

#[derive(Serialize)]
pub struct HelloResponse {
    pub server: String,
}
