import SwiftUI

struct FeatureTogglePanel: View {
    let features: [String]
    @Binding var enabledFeatures: [String: Bool]
    let onFeatureChanged: (String, Bool) -> Void

    var body: some View {
        Group {
            Text("Features").font(.subheadline).foregroundColor(.secondary)
            ForEach(features, id: \.self) { feature in
                Toggle(feature, isOn: Binding(
                    get: { enabledFeatures[feature, default: false] },
                    set: { enabled in
                        enabledFeatures[feature] = enabled
                        onFeatureChanged(feature, enabled)
                    }
                ))
            }
        }
    }
}
