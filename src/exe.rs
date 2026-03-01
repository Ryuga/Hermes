use tempfile::tempdir;
use crate::languages::get_handler;
use crate::loader::get_lang_config;
use crate::models::{Req, Resp};
use crate::runner::safe_execute;

pub fn execute_code(req: Req) -> Result<Resp, String>{
    let lang_config = get_lang_config(req.language.as_str());
    let handler = get_handler(req.language.as_str(), lang_config.clone());

    let work_dir = tempdir().map_err(|e| e.to_string())?;

    let program = match handler.prepare(work_dir.path(), &req.code) {
        Ok(p) => p,
        Err(e) => {
            return Ok(Resp{
                output: String::new(),
                std_log: e,
                code: 1,
                time_ms: 0
            });
        }
    };

    let exe_cmd = if lang_config.compile {
        format!("{} && {}", handler.compile_cmd(&program).join(" "), handler.run_cmd(&program).join(" "))
    }
    else {
        handler.run_cmd(&program).join(" ")
    };

    let (output, std_log, code, time_ms) = safe_execute(work_dir.path(), lang_config.clone(), exe_cmd)?;
    Ok(Resp{output, std_log, code, time_ms})
}
