import SwiftUI

struct ScreenshotPanel: View {
    let onSelectArea: () -> Void

    var body: some View {
        Group {
            Text("Screenshot").font(.subheadline).foregroundColor(.secondary)
            Button(action: onSelectArea) {
                HStack {
                    Image(systemName: "selection.pin.in.out")
                    Text("Select Area to Capture")
                }
                .frame(maxWidth: .infinity).padding(.vertical, 6)
            }
            .buttonStyle(.borderedProminent)
        }
    }
}
