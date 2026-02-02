use std::path::Path;
use std::process::{Command};
use crate::lang_config::LangConfig;

pub fn safe_execute(work_dir: &Path, config: LangConfig) -> Result<(String, String, i32), String> {
    let mut cmd = Command::new("nsjail");
    cmd.args([
        "--time_limit", "2",
        "--rlimit_as", "256",
        "--rlimit_cpu", "2",
        "--cgroup_mem_max 134217728",
        "--disable_proc",
        "--iface_no_lo",
        "--chroot",
        work_dir.to_str().unwrap(),
        "--",
    ]);

    cmd.args(config.run);
    cmd.current_dir(work_dir);
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let child = cmd.spawn().map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;

    Ok((
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
        out.status.code().unwrap_or(-1),
    ))
}