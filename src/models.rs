use serde::{Deserialize, Serialize};


fn default_compile() -> bool { false }
fn default_time_limit() -> u64 { 2 }
fn default_cpu_time_sec() -> u64 { 2 }
fn default_memory_mb() -> u64 { 256 }
fn default_stack_mb() -> u64 { 16 }
fn default_processes() -> u64 { 8 }
fn default_open_files() -> u64 { 64 }
fn default_output_kb() -> u64 { 1024 }

#[derive(Deserialize, Clone)]
pub struct LangConfig {
    pub source: String,

    pub run: Vec<String>,

    #[serde(default = "default_compile")]
    pub compile: bool,

    #[serde(default = "default_time_limit")]
    pub max_time_limit: u64,

    #[serde(default = "default_cpu_time_sec")]
    pub max_cpu_time_sec: u64,

    #[serde(default = "default_memory_mb")]
    pub max_memory_mb: u64,

    #[serde(default = "default_stack_mb")]
    pub max_stack_mb: u64,

    #[serde(default = "default_processes")]
    pub max_processes: u64,

    #[serde(default = "default_open_files")]
    pub max_open_files: u64,

    #[serde(default = "default_output_kb")]
    pub max_output_kb: u64,
}

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
