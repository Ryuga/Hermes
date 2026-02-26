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
    fn prepare(&self, work_dir: &Path, code: &str) -> Result<PreparedProgram, String>;
    fn compile(&self, prepared: &PreparedProgram) -> Result<(), String>;
}

pub fn get_handler(lang: &str, config: LangConfig) -> Box<dyn LanguageHandler> {
    match lang {
        "python" => Box::new(PythonHandler::new(config)),
        "javascript" => Box::new(JavascriptHandler::new(config)),
        _ => panic!("Unsupported language"),
    }
}
