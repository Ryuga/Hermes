use std::fs;
use std::path::Path;
use crate::languages::{LanguageHandler, PreparedProgram};
use crate::models::LangConfig;

pub struct PythonHandler;

impl LanguageHandler for PythonHandler {
    fn prepare(&self, work_dir: &Path, code: &str, config: &LangConfig) -> Result<PreparedProgram, String> {
        let file = work_dir.join(&config.source);
        fs::write(&file, code).map_err(|e| e.to_string())?;
        Ok(
            PreparedProgram {
                entry_file: file,
                run_cmd: config.run.clone(),
            }
        )
    }
    fn compile(&self, _: &PreparedProgram) -> Result<(), String> {
        println!("Ignoring compilation for python...");
        Ok(())
    }
}
