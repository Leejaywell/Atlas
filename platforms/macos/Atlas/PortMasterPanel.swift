import SwiftUI

struct PortMasterPanel: View {
    @State private var portInput: String = ""
    @State private var portError: String = ""

    var body: some View {
        Group {
            Text("Port Master").font(.subheadline).foregroundColor(.secondary)
            HStack {
                TextField("PID", text: $portInput).textFieldStyle(RoundedBorderTextFieldStyle())
                Button("Kill") {
                    guard let pid = UInt32(portInput) else {
                        portError = "Invalid: \"\(portInput)\""
                        return
                    }
                    portError = ""
                    if AtlasBridge.killPortProcess(pid: pid) { portInput = "" }
                }
                .disabled(portInput.isEmpty)
            }
            if !portError.isEmpty {
                Text(portError).font(.caption).foregroundColor(.red)
            }
        }
    }
}
