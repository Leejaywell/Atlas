import SwiftUI

// Mock/Placeholder for the UniFFI bridge
class AtlasBridge {
    static func listFeatures() -> [String] {
        // Simulating features exposed by Rust FeaturesManager
        return ["Logging", "Auto-Updates", "Experimental Mode"]
    }
    
    static func toggleFeature(name: String, enabled: Bool) {
        print("Feature \(name) toggled to \(enabled)")
        // This will call the actual Rust FFI in the future
    }
}

struct ContentView: View {
    @State private var statusText: String = "Initializing..."
    @State private var features: [String] = []
    @State private var enabledFeatures: [String: Bool] = [:]

    var body: some View {
        VStack(alignment: .leading, spacing: 20) {
            Text(statusText)
                .font(.headline)
            
            Divider()
            
            Text("Features")
                .font(.subheadline)
                .foregroundColor(.secondary)
            
            List {
                ForEach(features, id: \.self) { feature in
                    Toggle(feature, isOn: Binding(
                        get: { enabledFeatures[feature, default: false] },
                        set: { newValue in
                            enabledFeatures[feature] = newValue
                            AtlasBridge.toggleFeature(name: feature, enabled: newValue)
                        }
                    ))
                }
            }
            .frame(minHeight: 150)
            
            Divider()
            
            HStack {
                Button("Settings") {
                    NSApp.activate(ignoringOtherApps: true)
                }
                
                Spacer()
                
                Button("Quit") {
                    NSApplication.shared.terminate(nil)
                }
                .keyboardShortcut("q")
            }
        }
        .padding()
        .onAppear {
            features = AtlasBridge.listFeatures()
            statusText = "Atlas is Ready"
        }
    }
}

#Preview {
    ContentView()
}
