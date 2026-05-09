use sysinfo::Components;

use crate::monitor::models::TemperatureSnapshot;

/// 返回所有可用温度传感器的读数。
/// macOS 受 SIP 权限限制，可能返回空列表或仅部分传感器。
pub fn get_temperatures() -> Vec<TemperatureSnapshot> {
    Components::new_with_refreshed_list()
        .iter()
        .map(|c| TemperatureSnapshot {
            label: c.label().to_string(),
            celsius: c.temperature(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperatures_does_not_panic() {
        let temps = get_temperatures();
        for t in &temps {
            assert!(!t.label.is_empty());
            // macOS with SIP may return unrealistic values; just ensure we can read them
            // without panicking. Realistic range is roughly 0–150°C, but we don't assert it.
            let _ = t.celsius; // suppress unused warning if we have no assertions
        }
    }
}
