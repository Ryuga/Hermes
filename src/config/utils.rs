
use super::models::{ExecutionLimits, LangConfig};
use super::constants::LANG_CONFIG;


pub fn get_lang_config(language: &String, limits: &ExecutionLimits) -> Result<LangConfig, String> {
    let mut config: LangConfig = LANG_CONFIG
        .get(language)
        .ok_or_else(|| format!("Unsupported language: {}", language))?
        .clone();

    let apply_limit = |target: &mut i32, requested: Option<i32>, max: i32, err_msg: &'static str| -> Result<(), &'static str> {
        if let Some(val) = requested {
            if val <= 0 || val > max {
                return Err(err_msg);
            }
            *target = val;
        }
        Ok(())
    };

    apply_limit(&mut config.time_limit, limits.time_limit, config.max_time_limit, "Time limit invalid or exceeds max")?;
    apply_limit(&mut config.cpu_time_sec, limits.cpu_time_sec, config.max_cpu_time_sec, "CPU Time limit invalid or exceeds max")?;
    apply_limit(&mut config.memory_kb, limits.memory_kb, config.max_memory_kb, "Memory limit invalid or exceeds max")?;
    apply_limit(&mut config.stack_kb, limits.stack_kb, config.max_stack_kb, "Stack limit invalid or exceeds max")?;
    apply_limit(&mut config.open_files, limits.open_files, config.max_open_files, "Open files limit invalid or exceeds max")?;
    apply_limit(&mut config.file_size_kb, limits.file_size_kb, config.max_file_size_kb, "File size limit invalid or exceeds max")?;
    apply_limit(&mut config.processes, limits.processes, config.max_processes, "Processes limit invalid or exceeds max")?;

    Ok(config)
}
