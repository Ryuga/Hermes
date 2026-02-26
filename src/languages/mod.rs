pub mod python;
mod javascript;

use std::path::{Path, PathBuf};
use crate::languages::javascript::JavascriptHandler;
use crate::languages::python::PythonHandler;
use crate::models::LangConfig;

pub struct PreparedProgram {
    pub entry_file: PathBuf,
    pub run_cmd: Vec<String>,
}

pub trait LanguageHandler {
    fn prepare(&self, work_dir: &Path, code: &str, config: &LangConfig) -> Result<PreparedProgram, String>;
    fn compile(&self, prepared: &PreparedProgram) -> Result<(), String>;
}

pub fn get_handler(lang: &str) -> Box<dyn LanguageHandler> {
    match lang {
        "python" => Box::new(PythonHandler),
        "javascript" => Box::new(JavascriptHandler),
        _ => panic!("Unsupported language"),
    }
}
