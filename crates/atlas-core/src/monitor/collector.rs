use sysinfo::{System, Networks};
use crate::monitor::models::SystemSnapshot;

pub struct Collector {
    sys: System,
    networks: Networks,
}

impl Collector {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
            networks: Networks::new_with_refreshed_list(),
        }
    }

    pub fn take_snapshot(&mut self) -> SystemSnapshot {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        self.networks.refresh_list();

        let cpu_usage = self.sys.global_cpu_info().cpu_usage();
        let mem_used_bytes = self.sys.used_memory();
        let mem_total_bytes = self.sys.total_memory();

        let mut upload = 0;
        let mut download = 0;
        for (_, data) in &self.networks {
            upload += data.transmitted();
            download += data.received();
        }

        SystemSnapshot {
            cpu_usage,
            mem_used_bytes,
            mem_total_bytes,
            net_upload_bps: upload,
            net_download_bps: download,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_snapshot() {
        let mut collector = Collector::new();
        // First snapshot might have 0 CPU usage because it needs two refreshes with delay
        let _ = collector.take_snapshot();
        
        // Wait a bit or just take another one (sysinfo 0.30 usually works better with a small delay)
        // But for unit test we just want to see if it doesn't panic and returns memory.
        let snapshot = collector.take_snapshot();
        
        assert!(snapshot.mem_total_bytes > 0);
        println!("Snapshot: {:?}", snapshot);
    }
}
