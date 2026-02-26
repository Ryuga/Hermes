use std::fs;
use std::path::Path;
use crate::languages::{LanguageHandler, PreparedProgram};
use crate::models::LangConfig;

pub struct PythonHandler {
    config: LangConfig,
}

impl PythonHandler {
    pub fn new(config: LangConfig) -> Self {
        Self { config }
    }
}

impl LanguageHandler for PythonHandler {
    fn prepare(&self, work_dir: &Path, code: &str) -> Result<PreparedProgram, String> {
        let file = work_dir.join(&self.config.source);
        fs::write(&file, code).map_err(|e| e.to_string())?;
        Ok(
            PreparedProgram {
                entry_file: file,
                run_cmd: self.config.run.clone(),
            }
        )
    }
    fn compile(&self, _: &PreparedProgram) -> Result<(), String> {
        println!("Ignoring compilation for python...");
        Ok(())
    }
}
