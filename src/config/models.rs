use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

use crate::api::utils::Validate;

fn default_vector() -> Vec<String> { vec![] }
fn default_compile() -> bool { false }
fn default_authenticate() -> bool { false }
fn default_time_limit() ->  i32 {2}
fn default_cpu_time_limit() -> i32 {2}
fn default_memory_limit() -> i32 {256 * 1024}
fn default_stack_limit() -> i32 {64 * 1024}
fn default_processes_limit() -> i32 {16}
fn default_open_files_limit() -> i32 {64}
fn default_file_size_limit() -> i32 {1024}


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
    pub cpu_time_limit: i32,
    pub memory_limit: i32,
    pub stack_limit: i32,
    pub processes_limit: i32,
    pub open_files_limit: i32,
    pub file_size_limit: i32,

    pub max_time_limit: i32,
    pub max_cpu_time_limit: i32,
    pub max_memory_limit: i32,
    pub max_stack_limit: i32,
    pub max_processes_limit: i32,
    pub max_open_files_limit: i32,
    pub max_file_size_limit: i32,
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

    #[serde(default = "default_cpu_time_limit")]
    pub default_cpu_time_limit: i32,

    #[serde(default = "default_memory_limit")]
    pub default_memory_limit: i32,

    #[serde(default = "default_stack_limit")]
    pub default_stack_limit: i32,

    #[serde(default = "default_processes_limit")]
    pub default_processes_limit: i32,

    #[serde(default = "default_open_files_limit")]
    pub default_open_files_limit: i32,

    #[serde(default = "default_file_size_limit")]
    pub default_file_size_limit: i32,

    #[serde(default = "default_time_limit")]
    pub max_time_limit: i32,

    #[serde(default = "default_cpu_time_limit")]
    pub max_cpu_time_limit: i32,

    #[serde(default = "default_memory_limit")]
    pub max_memory_limit: i32,

    #[serde(default = "default_stack_limit")]
    pub max_stack_limit: i32,

    #[serde(default = "default_processes_limit")]
    pub max_processes_limit: i32,

    #[serde(default = "default_open_files_limit")]
    pub max_open_files_limit: i32,

    #[serde(default = "default_file_size_limit")]
    pub max_file_size_limit: i32,

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
                cpu_time_limit: raw.default_cpu_time_limit,
                memory_limit: raw.default_memory_limit,
                stack_limit: raw.default_stack_limit,
                processes_limit: raw.default_processes_limit,
                open_files_limit: raw.default_open_files_limit,
                file_size_limit: raw.default_file_size_limit,

                max_time_limit: raw.max_time_limit,
                max_cpu_time_limit: raw.max_cpu_time_limit,
                max_memory_limit: raw.max_memory_limit,
                max_stack_limit: raw.max_stack_limit,
                max_processes_limit: raw.max_processes_limit,
                max_open_files_limit: raw.max_open_files_limit,
                max_file_size_limit: raw.max_file_size_limit,
            })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExecutionLimits {
    pub time_limit: Option<i32>,
    pub cpu_time_limit: Option<i32>,
    pub memory_limit: Option<i32>,
    pub stack_limit: Option<i32>,
    pub processes_limit: Option<i32>,
    pub open_files_limit: Option<i32>,
    pub file_size_limit: Option<i32>,
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
