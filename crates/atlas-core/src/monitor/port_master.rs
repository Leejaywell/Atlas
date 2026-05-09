use crate::monitor::models::PortProcessInfo;
use anyhow::{Context, Result};
use std::process::Command;
use sysinfo::{Pid, System};

/// Finds process information associated with a specific TCP port in LISTEN state.
///
/// This function relies on the Unix `lsof` command to identify the process listening
/// on the given port. It requires `lsof` to be installed and available in the PATH.
/// It then uses the `sysinfo` library to retrieve process details.
pub fn find_process_by_port(port: u16) -> Result<Option<PortProcessInfo>> {
    // Run lsof to get the PID of the process listening on the port
    // -i :<port> filters by port
    // -t returns only the PID
    // -sTCP:LISTEN filters for listening state
    let output = Command::new("lsof")
        .args(["-i", &format!(":{}", port), "-t", "-sTCP:LISTEN"])
        .output()
        .context("Failed to execute lsof command. Please ensure 'lsof' is installed.")?;

    let pid_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if pid_str.is_empty() {
        return Ok(None);
    }

    // lsof might return multiple PIDs if multiple processes are involved,
    // but typically for LISTEN it's one. We take the first one.
    let first_pid_str = pid_str
        .lines()
        .next()
        .context("Failed to get PID from lsof output")?
        .trim();
    let pid: u32 = first_pid_str
        .parse()
        .with_context(|| format!("Failed to parse PID '{}' from lsof output", first_pid_str))?;

    // Use sysinfo to get the process name instead of spawning `ps`
    let mut sys = System::new();
    let sys_pid = Pid::from(pid as usize);
    sys.refresh_process(sys_pid);

    let process_name = if let Some(process) = sys.process(sys_pid) {
        process.name().to_string()
    } else {
        "unknown".to_string()
    };

    Ok(Some(PortProcessInfo {
        port,
        pid,
        process_name,
    }))
}

/// Attempts to kill a process with the given PID using `kill -9`.
pub fn kill_process(pid: u32) -> bool {
    Command::new("kill")
        .args(["-9", &pid.to_string()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn test_find_process_by_port() {
        // Bind to an ephemeral port
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to ephemeral port");
        let port = listener.local_addr().unwrap().port();

        let info = find_process_by_port(port).expect("Should not fail");
        assert!(
            info.is_some(),
            "Should find the process listening on port {}",
            port
        );

        let info = info.unwrap();
        assert_eq!(info.port, port);
        assert!(info.pid > 0);
        // On macOS, process_name might be the full path or just the binary name.
        // It's hard to assert exactly what it will be, but it shouldn't be empty.
        assert!(!info.process_name.is_empty());
    }

    #[test]
    fn test_find_process_non_existent_port() {
        // Use a port that is unlikely to be in use
        let info = find_process_by_port(65534).expect("Should not fail");
        assert!(info.is_none());
    }

    #[test]
    fn test_kill_process() {
        use std::process::Stdio;
        // Start a process that stays alive for a bit
        let mut child = Command::new("sleep")
            .arg("10")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("Failed to spawn sleep");
        let pid = child.id();

        let success = kill_process(pid);
        assert!(success);

        // Wait a bit for it to be killed
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Check if it's dead
        let status = child.try_wait().unwrap();
        assert!(status.is_some());
    }
}
