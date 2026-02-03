use std::path::Path;
use std::process::{Command};
use crate::lang_config::LangConfig;

pub fn safe_execute(work_dir: &Path, config: LangConfig) -> Result<(String, String, i32), String> {
    let mut cmd = Command::new("sudo");
    cmd.arg("nsjail");
    cmd.args([
        "--time_limit", "2",
        "--rlimit_as", "256",
        "--rlimit_cpu", "2",
        "--disable_proc",
        "--iface_no_lo",
        "--bindmount_ro", "/usr",
        "--bindmount_ro", "/lib",
        "--bindmount_ro", "/lib64",
        "--bindmount_ro", "/bin",
        "--bindmount",
        "--bindmount", &format!("{}:/", work_dir.to_str().unwrap()),
        "--chroot", "/",
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