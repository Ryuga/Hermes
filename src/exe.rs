use std::fs;
use axum::Json;
use tempfile::tempdir;
use crate::lang_config::get_lang_config;
use crate::models::{Req, Resp};
use crate::runner::safe_execute;

pub async fn execute_code(req: Req) -> Result<String, String>{
    let lang_config = get_lang_config(req.language.as_str());
    let work_dir = tempdir().map_err(|e| e.to_string())?;

    let file = work_dir.path().join(lang_config.source);
    fs::write(&file, req.code).map_err(|e| e.to_string())?;
    let (output, std_log, code) = safe_execute(work_dir.path(), lang_config)?;

    Ok(Json(Resp {code, output, std_log}))
}