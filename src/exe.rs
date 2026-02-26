use tempfile::tempdir;
use crate::languages::get_handler;
use crate::loader::get_lang_config;
use crate::models::{Req, Resp};
use crate::runner::safe_execute;

pub fn execute_code(req: Req) -> Result<Resp, String>{
    let lang_config = get_lang_config(req.language.as_str());
    let handler = get_handler(req.language.as_str(), lang_config.clone());

    let work_dir = tempdir().map_err(|e| e.to_string())?;

    let program = handler.prepare(work_dir.path(), &req.code)?;

    if lang_config.compile {
        unimplemented!()
    }

    let mut exec_config = lang_config.clone();
    exec_config.run = program.run_cmd.clone();

    let (output, std_log, code, time_ms) = safe_execute(work_dir.path(), exec_config)?;
    Ok(Resp{output, std_log, code, time_ms})
}
