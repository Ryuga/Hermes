
use super::models::{ExecutionLimits, LangConfig};
use super::constants::LANG_CONFIG;


pub fn get_lang_config(language: &String, limits: Option<&ExecutionLimits>) -> Result<LangConfig, String> {
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
    if let Some(limits) = limits {
        apply_limit(&mut config.time_limit, limits.time_limit, config.max_time_limit, "Time limit invalid or exceeds max")?;
        apply_limit(&mut config.cpu_time_limit, limits.cpu_time_limit, config.max_cpu_time_limit, "CPU Time limit invalid or exceeds max")?;
        apply_limit(&mut config.memory_limit, limits.memory_limit, config.max_memory_limit, "Memory limit invalid or exceeds max")?;
        apply_limit(&mut config.stack_limit, limits.stack_limit, config.max_stack_limit, "Stack limit invalid or exceeds max")?;
        apply_limit(&mut config.open_files_limit, limits.open_files_limit, config.max_open_files_limit, "Open files limit invalid or exceeds max")?;
        apply_limit(&mut config.file_size_limit, limits.file_size_limit, config.max_file_size_limit, "File size limit invalid or exceeds max")?;
        apply_limit(&mut config.processes_limit, limits.processes_limit, config.max_processes_limit, "Processes limit invalid or exceeds max")?;
    }

    Ok(config)
}
