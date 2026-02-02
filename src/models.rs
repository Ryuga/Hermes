use serde::Deserialize;

#[derive(Deserialize)]
pub struct Req {
    pub language: String,
    pub code: String
}