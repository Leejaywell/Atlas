//! Atlas FFI Crate
//!
//! This crate provides a Foreign Function Interface (FFI) for the Atlas core functionality,
//! allowing it to be used from other languages via UniFFI.

use std::sync::Mutex;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use atlas_core::AtlasCore;
use thiserror::Error;

uniffi::include_scaffolding!("atlas");

/// Global instance of the Atlas core to preserve state across FFI calls.
static CORE: Lazy<Mutex<AtlasCore>> = Lazy::new(|| Mutex::new(AtlasCore::new()));

/// Control the monitoring background task.
static MONITOR_HANDLE: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

/// Global Tokio runtime for background tasks.
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Failed to create Tokio runtime"));

#[derive(Debug, Error)]
pub enum AtlasError {
    #[error("Atlas Core lock is poisoned")]
    LockPoisoned,
    #[error("Monitoring error: {0}")]
    MonitoringError(String),
    #[error("Process error: {0}")]
    ProcessError(String),
}

/// Represents the state of a feature module for FFI.
pub enum FeatureStatus {
    Enabled,
    Disabled,
}

/// A record representing a feature and its current status for FFI.
pub struct FeatureEntry {
    pub name: String,
    pub status: FeatureStatus,
}

/// A snapshot of system performance metrics for FFI.
pub struct SystemSnapshot {
    pub cpu_usage: f32,
    pub mem_used_bytes: u64,
    pub mem_total_bytes: u64,
    pub net_upload_bps: u64,
    pub net_download_bps: u64,
}

/// Information about a process associated with a network port for FFI.
pub struct PortProcessInfo {
    pub port: u16,
    pub pid: u32,
    pub process_name: String,
}

pub trait SystemMonitorCallback: Send + Sync {
    fn on_snapshot(&self, snapshot: SystemSnapshot);
}

impl From<atlas_core::features::FeatureStatus> for FeatureStatus {
    fn from(status: atlas_core::features::FeatureStatus) -> Self {
        match status {
            atlas_core::features::FeatureStatus::Enabled => Self::Enabled,
            atlas_core::features::FeatureStatus::Disabled => Self::Disabled,
        }
    }
}

/// Returns the current status of the Atlas core.
///
/// This function uses the global `AtlasCore` instance.
pub fn get_core_status() -> Result<String, AtlasError> {
    let core = CORE.lock().map_err(|_| AtlasError::LockPoisoned)?;
    Ok(core.get_status())
}

/// Toggles a feature state.
///
/// Returns true if the feature existed and was toggled.
pub fn toggle_feature(name: String, enabled: bool) -> Result<bool, AtlasError> {
    let mut core = CORE.lock().map_err(|_| AtlasError::LockPoisoned)?;
    Ok(core.feature_manager_mut()
        .toggle_feature(&name, enabled))
}

/// Returns a list of all available features and their status.
pub fn list_features() -> Result<Vec<FeatureEntry>, AtlasError> {
    let core = CORE.lock().map_err(|_| AtlasError::LockPoisoned)?;
    Ok(core.feature_manager()
        .list_features()
        .into_iter()
        .map(|(name, status)| FeatureEntry {
            name,
            status: status.into(),
        })
        .collect())
}

/// Starts real-time system monitoring.
///
/// This spawns a background task that collects system metrics every second
/// and pushes them to the provided callback. If monitoring is already active,
/// the existing task is stopped before starting a new one.
pub fn start_monitoring(callback: Box<dyn SystemMonitorCallback>) -> Result<(), AtlasError> {
    let mut handle_lock = MONITOR_HANDLE.lock().map_err(|_| AtlasError::LockPoisoned)?;
    
    // Stop existing task if any
    if let Some(handle) = handle_lock.take() {
        handle.abort();
    }

    let handle = RUNTIME.spawn(async move {
        let mut collector = atlas_core::monitor::collector::Collector::new();
        loop {
            let snapshot = collector.take_snapshot();
            callback.on_snapshot(SystemSnapshot {
                cpu_usage: snapshot.cpu_usage,
                mem_used_bytes: snapshot.mem_used_bytes,
                mem_total_bytes: snapshot.mem_total_bytes,
                net_upload_bps: snapshot.net_upload_bps,
                net_download_bps: snapshot.net_download_bps,
            });
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    *handle_lock = Some(handle);
    Ok(())
}

/// Stops real-time system monitoring.
pub fn stop_monitoring() -> Result<(), AtlasError> {
    let mut handle_lock = MONITOR_HANDLE.lock().map_err(|_| AtlasError::LockPoisoned)?;
    if let Some(handle) = handle_lock.take() {
        handle.abort();
    }
    Ok(())
}

/// Looks up process information for a specific TCP port.
pub fn lookup_port(port: u16) -> Option<PortProcessInfo> {
    atlas_core::monitor::port_master::find_process_by_port(port)
        .ok()
        .flatten()
        .map(|info| PortProcessInfo {
            port: info.port,
            pid: info.pid,
            process_name: info.process_name,
        })
}

/// Kills a process by its PID.
pub fn kill_port_process(pid: u32) -> bool {
    atlas_core::monitor::port_master::kill_process(pid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_core_status() {
        let status = get_core_status().unwrap();
        assert!(status.contains("Atlas Core v"));
        assert!(status.contains("is running"));
    }

    #[test]
    fn test_feature_management() {
        // Verify default features exist
        let features = list_features().unwrap();
        assert!(features.iter().any(|f| f.name == "monitoring"));
        assert!(features.iter().any(|f| f.name == "screenshot"));
        
        // Verify toggle returns true for existing feature
        assert!(toggle_feature("monitoring".to_string(), true).unwrap());
        
        // Verify toggle returns false for non-existent feature
        assert!(!toggle_feature("non-existent".to_string(), true).unwrap());
    }

    #[test]
    fn test_port_lookup() {
        use std::net::TcpListener;
        // Bind to an ephemeral port to test lookup
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind");
        let port = listener.local_addr().unwrap().port();
        
        let info = lookup_port(port);
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(info.port, port);
        assert!(info.pid > 0);
    }
}
