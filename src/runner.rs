use std::env;
use std::path::Path;
use std::process::Command;
use tokio::time::Instant;
use crate::models::LangConfig;


pub fn safe_execute(work_dir: &Path, config: LangConfig, run_cmd: String) -> Result<(String, String, i32, u128), String> {
    let start = Instant::now();
    let debug = env::var("DEBUG").unwrap_or_else(|_|"false".to_string()) == "true";
    let fsize_mb = ((config.max_output_kb + 1023) / 1024).max(1);
    let tmpfs_bytes = 16 * 1024 * 1024;

    let mut cmd = Command::new("sudo");

    cmd.arg("nsjail");

    cmd.args([
        "--mode", "o",
        "--experimental_mnt", "old", // since new api is getting rejected.
        "--cwd", "/sandbox",
        "--max_cpus", "1",
        "--user", "99999",
        "--group", "99999",
    ]);

    if  !debug {
        cmd.arg("--really_quiet");
    }

    // TODO: rootless running not possible currently since mount not supported.
    // cmd.arg("--uid_mapping").arg(format!("0:{}:1", nix::unistd::getuid()));
    // cmd.arg("--gid_mapping").arg(format!("0:{}:1", nix::unistd::getgid()));

    // Limit enforcement per language
    cmd.arg("--time_limit").arg(config.max_time_limit.to_string());
    cmd.arg("--rlimit_as").arg(config.max_memory_mb.to_string());
    cmd.arg("--rlimit_cpu").arg(config.max_cpu_time_sec.to_string());
    cmd.arg("--rlimit_nofile").arg(config.max_open_files.to_string());
    cmd.arg("--rlimit_nproc").arg(config.max_processes.to_string());
    cmd.arg("--rlimit_stack").arg(config.max_stack_mb.to_string());
    cmd.arg("--rlimit_fsize").arg(fsize_mb.to_string());

    cmd.arg("--bindmount_ro").arg("/dev/null:/dev/null");

    // Read only input and copy to temp mount as last step for exec
    cmd.arg("--bindmount_ro")
        .arg(format!("{}:/input", work_dir.display()));


    // TODO: New API was getting rejected on Oracle VM. Need to check on local
    // Use old binding API
    // cmd.arg("--tmpfsmount")
    //     .arg("/sandbox:size=16m");

    cmd.arg("--mount")
        .arg(format!("none:/sandbox:tmpfs:size={}", tmpfs_bytes));



    // Mount system libs
    cmd.args([
        "--bindmount_ro", "/usr",
        "--bindmount_ro", "/lib",
        "--bindmount_ro", "/lib64",
        "--bindmount_ro", "/bin",
        "--bindmount_ro", "/etc", // needed for java
    ]);

    // Added for Java exec since dynamic link to libjli.so is not resolved.
    cmd.arg("--bindmount_ro")
        .arg("/usr/lib/jvm");

    // Interface Isolation
    cmd.args([
        "--disable_proc",
        "--iface_no_lo",
        "--env", "PATH=/usr/bin:/bin",
    ]);

    let wrapped = format!("cp -r /input/* /sandbox/ 2>/dev/null; {}", run_cmd);

    cmd.args([
        "--",
        "/bin/sh",
        "-c",
        &wrapped,
    ]);

    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let child = cmd.spawn().map_err(|e| e.to_string())?;
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    let time_ms = start.elapsed().as_millis();

    Ok((
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
        out.status.code().unwrap_or(-1),
        time_ms,
    ))
}
