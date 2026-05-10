import SwiftUI

struct AppFooter: View {
    var body: some View {
        HStack {
            Button("Settings") { NSApp.activate(ignoringOtherApps: true) }
            Spacer()
            Button("Quit") { NSApplication.shared.terminate(nil) }.keyboardShortcut("q")
        }
    }
}
