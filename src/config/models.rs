use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

use crate::api::utils::Validate;

fn default_vector() -> Vec<String> { vec![] }
fn default_compile() -> bool { false }
fn default_authenticate() -> bool { false }
fn default_time_limit() ->  i32 {2}
fn default_cpu_time_sec() -> i32 {2}
fn default_memory_kb() -> i32 {256 * 1024}
fn default_stack_kb() -> i32 {64 * 1024}
fn default_processes() -> i32 {16}
fn default_open_files() -> i32 {64}
fn default_file_size_kb() -> i32 {1024}


#[derive(Clone, Debug)]
pub struct LangConfig {
    pub source: String,

    pub compile: bool,

    pub authenticate: bool,

    pub compiler_path: String,
    pub compiler_args: Vec<String>,

    pub runtime_path: String,
    pub runtime_args: Vec<String>,
    pub isolate_args: Vec<String>,

    pub time_limit: i32,
    pub cpu_time_sec: i32,
    pub memory_kb: i32,
    pub stack_kb: i32,
    pub processes: i32,
    pub open_files: i32,
    pub file_size_kb: i32,

    pub max_time_limit: i32,
    pub max_cpu_time_sec: i32,
    pub max_memory_kb: i32,
    pub max_stack_kb: i32,
    pub max_processes: i32,
    pub max_open_files: i32,
    pub max_file_size_kb: i32,
}

#[derive(Deserialize)]
pub struct RawLangConfig {
    pub source: String,

    #[serde(default = "default_compile")]
    pub compile: bool,

    #[serde(default = "default_authenticate")]
    pub authenticate: bool,

    #[serde(default)]
    pub compiler_path: Option<String>,

    #[serde(default = "default_vector")]
    pub compiler_args: Vec<String>,

    pub runtime_path: String,

    #[serde(default = "default_vector")]
    pub runtime_args: Vec<String>,

    #[serde(default = "default_vector")]
    pub isolate_args: Vec<String>,

    #[serde(default = "default_time_limit")]
    pub default_time_limit: i32,

    #[serde(default = "default_cpu_time_sec")]
    pub default_cpu_time_sec: i32,

    #[serde(default = "default_memory_kb")]
    pub default_memory_kb: i32,

    #[serde(default = "default_stack_kb")]
    pub default_stack_kb: i32,

    #[serde(default = "default_processes")]
    pub default_processes: i32,

    #[serde(default = "default_open_files")]
    pub default_open_files: i32,

    #[serde(default = "default_file_size_kb")]
    pub default_file_size_kb: i32,

    #[serde(default = "default_time_limit")]
    pub max_time_limit: i32,

    #[serde(default = "default_cpu_time_sec")]
    pub max_cpu_time_sec: i32,

    #[serde(default = "default_memory_kb")]
    pub max_memory_kb: i32,

    #[serde(default = "default_stack_kb")]
    pub max_stack_kb: i32,

    #[serde(default = "default_processes")]
    pub max_processes: i32,

    #[serde(default = "default_open_files")]
    pub max_open_files: i32,

    #[serde(default = "default_file_size_kb")]
    pub max_file_size_kb: i32,

}


impl<'de> Deserialize<'de> for LangConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>,
        {
            let raw = RawLangConfig::deserialize(deserializer)?;

            if raw.source.trim().is_empty(){
                return Err(D::Error::custom("source can't be empty"));
            }

            if raw.compile && raw.compiler_path.is_none() {
                return Err(D::Error::custom(
                    "compiler_path is required when compile is set true",
                ));
            }

            Ok(LangConfig {
                source: raw.source,
                compile: raw.compile,
                authenticate: raw.authenticate,
                runtime_path: raw.runtime_path,
                compiler_path: raw.compiler_path.unwrap_or_default(),
                compiler_args: raw.compiler_args,
                runtime_args: raw.runtime_args,
                isolate_args: raw.isolate_args,

                time_limit: raw.default_time_limit,
                cpu_time_sec: raw.default_cpu_time_sec,
                memory_kb: raw.default_memory_kb,
                stack_kb: raw.default_stack_kb,
                processes: raw.default_processes,
                open_files: raw.default_open_files,
                file_size_kb: raw.default_file_size_kb,

                max_time_limit: raw.max_time_limit,
                max_cpu_time_sec: raw.max_cpu_time_sec,
                max_memory_kb: raw.max_memory_kb,
                max_stack_kb: raw.max_stack_kb,
                max_processes: raw.max_processes,
                max_open_files: raw.max_open_files,
                max_file_size_kb: raw.max_file_size_kb,
            })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExecutionLimits {
    pub time_limit: Option<i32>,
    pub cpu_time_sec: Option<i32>,
    pub memory_kb: Option<i32>,
    pub stack_kb: Option<i32>,
    pub processes: Option<i32>,
    pub open_files: Option<i32>,
    pub file_size_kb: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Req {
    pub language: String,
    pub code: String,
    pub limits: Option<ExecutionLimits>,
}

#[derive(Serialize, Debug)]
pub struct Resp {
    pub code: i32,
    pub output: String,
    pub std_log: String,
    pub time_ms: u128,
}

#[derive(Deserialize, Debug, Clone)]
pub struct File{
    pub name: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ReqMulti {
    pub language: String,
    pub files: Vec<File>,
    pub entry_file: String,
    pub limits: Option<ExecutionLimits>,
}

impl Validate for ReqMulti {
    fn validate(&self) -> Result<(), String> {

        let mut entry_file_found = false;

        if self.files.is_empty() {
            return Err("Preparation Error: No files provided".into());
        }

        for file in &self.files {
            if file.name.contains("..") || file.name.starts_with('/') {
                return Err(
                    format!("Security Violation: Invalid path in filename '{}'", file.name)
                );
            }
            if file.name == self.entry_file {
                entry_file_found = true;
            }
        }

        if !entry_file_found {
            return Err(
                format!(
                    "Preparation Error: Entry file '{}' not found in file list",
                    self.entry_file
                )
            );
        }

        Ok(())
    }
}
