use sysinfo::Disks;

use crate::monitor::models::DiskSnapshot;

/// 返回所有已挂载卷的空间信息。
pub fn get_disk_info() -> Vec<DiskSnapshot> {
    Disks::new_with_refreshed_list()
        .iter()
        .map(|disk| {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);
            DiskSnapshot {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_bytes: total,
                used_bytes: used,
                available_bytes: available,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_info_non_empty() {
        let disks = get_disk_info();
        assert!(!disks.is_empty(), "Should find at least one disk volume");
    }

    #[test]
    fn test_disk_used_plus_available_equals_total() {
        for disk in get_disk_info() {
            assert_eq!(
                disk.used_bytes + disk.available_bytes,
                disk.total_bytes,
                "used + available should equal total for {}",
                disk.name
            );
        }
    }
}
