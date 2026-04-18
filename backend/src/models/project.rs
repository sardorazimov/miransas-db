use serde::Deserialize;
#[derive(Deserialize)]
pub struct ProjectRequest {
    pub name: String,
}