use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Req {
    pub language: String,
    pub code: String
}

#[derive(Serialize)]
pub struct Resp {
    pub code: i32,
    pub output: String,
    pub std_log: String
}