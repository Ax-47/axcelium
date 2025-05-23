use std::collections::HashSet;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct CreateRolePayload {
    pub name: String,
    pub description: Option<String>,
    pub permissions: HashSet<String>,
}
