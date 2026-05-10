import Foundation

struct CpuCoreSnapshot {
    let name: String
    let usage: Float
    let frequencyMhz: UInt64
}

struct ProcessSnapshot {
    let pid: UInt32
    let name: String
    let cpuUsage: Float
    let memBytes: UInt64
}

struct NetworkInterfaceSnapshot {
    let name: String
    let uploadBps: UInt64
    let downloadBps: UInt64
}

struct DiskSnapshot {
    let name: String
    let mountPoint: String
    let totalBytes: UInt64
    let usedBytes: UInt64
    let availableBytes: UInt64
}

struct BatterySnapshot {
    let chargePercent: Float
    let isCharging: Bool
    let timeToEmptySecs: Int64?
    let timeToFullSecs: Int64?
    let healthPercent: Float
    let cycleCount: UInt32?
}

struct TemperatureSnapshot {
    let label: String
    let celsius: Float
}

struct SystemSnapshot {
    let cpuUsage: Float
    let memUsedBytes: UInt64
    let memTotalBytes: UInt64
    let netUploadBps: UInt64
    let netDownloadBps: UInt64
    let cpuCores: [CpuCoreSnapshot]
    let memFreeBytes: UInt64
    let memAvailableBytes: UInt64
    let swapUsedBytes: UInt64
    let swapTotalBytes: UInt64
    let topCpuProcesses: [ProcessSnapshot]
    let topMemProcesses: [ProcessSnapshot]
    let networkInterfaces: [NetworkInterfaceSnapshot]
    let disks: [DiskSnapshot]
    let battery: BatterySnapshot?
    let temperatures: [TemperatureSnapshot]
}

enum Formatters {
    static func bytes(_ bytes: UInt64) -> String {
        ByteCountFormatter.string(fromByteCount: Int64(bytes), countStyle: .memory)
    }

    static func speed(_ bps: UInt64) -> String {
        ByteCountFormatter.string(fromByteCount: Int64(bps), countStyle: .file) + "/s"
    }

    static func time(_ secs: Int64) -> String {
        let h = secs / 3600
        let m = (secs % 3600) / 60
        return h > 0 ? "\(h)h \(m)m" : "\(m)m"
    }
}
