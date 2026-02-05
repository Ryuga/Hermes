use std::env;
use std::path::Path;
use std::process::Command;
use crate::models::LangConfig;


pub fn safe_execute(work_dir: &Path, config: LangConfig) -> Result<(String, String, i32), String> {
    let debug = env::var("DEBUG").unwrap_or_else(|_|"false".to_string()) == "true";
    let fsize_mb = ((config.max_output_kb + 1023) / 1024).max(1);

    let mut cmd = Command::new("nsjail");

    cmd.args([
        "--mode", "o",
        "--cwd", "/sandbox",
        "--max_cpus", "1",
    ]);

    if  !debug {
        cmd.arg("--really_quiet");
    }
    cmd.arg("--uid_mapping").arg(format!("0:{}:1", nix::unistd::getuid()));
    cmd.arg("--gid_mapping").arg(format!("0:{}:1", nix::unistd::getgid()));

    cmd.arg("--time_limit").arg(config.max_time_limit.to_string());
    cmd.arg("--rlimit_as").arg(config.max_memory_mb.to_string());
    cmd.arg("--rlimit_cpu").arg(config.max_cpu_time_sec.to_string());
    cmd.arg("--rlimit_nofile").arg(config.max_open_files.to_string());
    cmd.arg("--rlimit_nproc").arg(config.max_processes.to_string());
    cmd.arg("--rlimit_stack").arg(config.max_stack_mb.to_string());
    cmd.arg("--rlimit_fsize").arg(fsize_mb.to_string());

    cmd.arg("--bindmount").arg(format!("{}:/sandbox", work_dir.display()));



    cmd.args([
        "--bindmount_ro", "/usr",
        "--bindmount_ro", "/lib",
        "--bindmount_ro", "/lib64",
        "--bindmount_ro", "/bin",
        "--disable_proc",
        "--iface_no_lo",
        "--env", "PATH=/usr/bin:/bin",
        "--",
    ]);



    cmd.args(&config.run);

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
