import SwiftUI

struct SelectionOverlay: View {
    @State private var startPoint: CGPoint?
    @State private var endPoint: CGPoint?
    var onCapture: (CGRect) -> Void

    var body: some View {
        ZStack(alignment: .topLeading) {
            Color.black.opacity(0.3)
                .edgesIgnoringSafeArea(.all)
                .onTapGesture {
                    // Tap background to cancel? Or handled elsewhere.
                }
            
            if let start = startPoint, let end = endPoint {
                let rect = selectionRect(start: start, end: end)
                
                Rectangle()
                    .fill(Color.white.opacity(0.01)) // Make it interactive
                    .border(Color.blue, width: 2)
                    .frame(width: rect.width, height: rect.height)
                    .offset(x: rect.minX, y: rect.minY)
            }
        }
        .gesture(
            DragGesture(minimumDistance: 0)
                .onChanged { value in
                    if startPoint == nil { startPoint = value.startLocation }
                    endPoint = value.location
                }
                .onEnded { value in
                    if let start = startPoint {
                        let rect = CGRect(
                            x: min(start.x, value.location.x),
                            y: min(start.y, value.location.y),
                            width: abs(start.x - value.location.x),
                            height: abs(start.y - value.location.y)
                        )
                        if rect.width > 5 && rect.height > 5 {
                            onCapture(rect)
                        }
                    }
                    startPoint = nil
                    endPoint = nil
                }
        )
    }

    private func selectionRect(start: CGPoint, end: CGPoint) -> CGRect {
        CGRect(x: min(start.x, end.x),
               y: min(start.y, end.y),
               width: abs(start.x - end.x),
               height: abs(start.y - end.y))
    }
}

#Preview {
    SelectionOverlay { rect in
        print("Captured: \(rect)")
    }
}
