use serde::Serialize;

#[derive(Serialize)]
pub struct HelloJSON {
    pub server: String,
}
