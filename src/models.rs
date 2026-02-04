use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Req {
    pub language: String,
    pub code: String
}

#[derive(Serialize)]
pub struct Resp {
    pub code: i8,
    pub output: String,
    pub std_log: String
}