import SwiftUI

@main
struct AtlasApp: App {
    @State private var statusText: String = "Initializing..."

    var body: some Scene {
        MenuBarExtra("Atlas", systemImage: "square.stack.3d.up.fill") {
            VStack {
                Text(statusText)
                    .font(.headline)
                
                Divider()
                
                Button("Settings") {
                    NSApp.activate(ignoringOtherApps: true)
                    // Opening settings UI will be implemented later
                }
                
                Button("Quit") {
                    NSApplication.shared.terminate(nil)
                }
                .keyboardShortcut("q")
            }
            .padding()
            .onAppear {
                // This will eventually call the Rust Core via the UniFFI bridge
                statusText = "Atlas is Ready"
            }
        }
    }
}
