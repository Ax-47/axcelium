use serde::Serialize;

#[derive(Serialize)]
pub struct Hello {
    pub server: String,
}
