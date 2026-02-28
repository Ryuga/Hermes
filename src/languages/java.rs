use std::fs;
use std::path::Path;
use regex::Regex;
use crate::languages::{LanguageHandler, PreparedProgram};
use crate::models::LangConfig;

pub struct JavaHandler {
    config: LangConfig,
}

impl JavaHandler {
    pub fn new(config: LangConfig) -> Self {
        Self { config }
    }

    fn extract_main_class_name(code: &str) -> Result<String, String> {
        let class_re =
            Regex::new(r"(?m)^\s*public\s+(?:\w+\s+)*class\s+([A-Za-z_][A-Za-z0-9_]*)")
                .unwrap();
        let res = class_re.captures(&code).ok_or("public class not found!")?;
        Ok(res[1].to_string())
    }
}

impl LanguageHandler for JavaHandler {
    fn prepare(&self, work_dir: &Path, code: &str) -> Result<PreparedProgram, String> {
        let class_name = Self::extract_main_class_name(&code)?;

        let file_path = work_dir.join(format!("{}.java", class_name));
        fs::write(&file_path, code).map_err(|e| e.to_string())?;

        Ok(
            PreparedProgram {
                entry_file: file_path,
                entry_name: class_name,
            }
        )
    }
    fn compile_cmd(&self, prepared: &PreparedProgram) -> Vec<String> {
        let mut cmd = vec![self.config.compiler_path.clone()];
        cmd.extend(self.config.compiler_args.clone());
        cmd.push(format!("{}.java", prepared.entry_name));
        cmd
    }

    fn run_cmd(&self, prepared: &PreparedProgram) -> Vec<String> {
        let mut cmd = vec![self.config.runtime_path.clone()];
        cmd.extend(self.config.runtime_args.clone());
        cmd.push(prepared.entry_name.clone());
        cmd
    }
}
