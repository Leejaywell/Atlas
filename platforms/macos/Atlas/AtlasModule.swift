enum AtlasModule: String, CaseIterable, Identifiable {
    case screenshot
    case monitoring

    var id: String { rawValue }

    var featureName: String {
        rawValue
    }

    var title: String {
        switch self {
        case .screenshot:
            return "Screenshot"
        case .monitoring:
            return "Monitoring"
        }
    }
}
