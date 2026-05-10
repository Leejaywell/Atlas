import AppKit
import XCTest
@testable import Atlas

final class ScreenshotEditorRendererTests: XCTestCase {
    func testRenderedPNGIncludesAnnotations() throws {
        let data = try XCTUnwrap(AtlasBridge.captureRegion(x: 0, y: 0, width: 120, height: 80))
        let screenshot = CapturedScreenshot(
            pngData: data,
            rect: CGRect(x: 0, y: 0, width: 120, height: 80)
        )
        let annotations: [ScreenshotAnnotation] = [
            .rectangle(rect: CGRect(x: 8, y: 8, width: 40, height: 24), color: .red, lineWidth: 2),
            .arrow(from: CGPoint(x: 12, y: 60), to: CGPoint(x: 80, y: 24), color: .red, lineWidth: 2),
            .pen(points: [CGPoint(x: 70, y: 60), CGPoint(x: 90, y: 64), CGPoint(x: 110, y: 52)], color: .red, lineWidth: 2),
            .text(value: "Text", rect: CGRect(x: 18, y: 38, width: 48, height: 18), color: .red),
            .pixelate(rect: CGRect(x: 78, y: 10, width: 28, height: 24)),
        ]

        let rendered = ScreenshotEditorRenderer.renderedPNGData(
            screenshot: screenshot,
            annotations: annotations,
            canvasSize: CGSize(width: 120, height: 80)
        )
        let bitmap = try XCTUnwrap(NSBitmapImageRep(data: rendered))

        XCTAssertNotEqual(rendered, data)
        XCTAssertEqual(bitmap.pixelsWide, 120)
        XCTAssertEqual(bitmap.pixelsHigh, 80)
    }

    func testRenderedPNGReturnsOriginalWithoutAnnotations() throws {
        let data = try XCTUnwrap(AtlasBridge.captureRegion(x: 0, y: 0, width: 64, height: 48))
        let screenshot = CapturedScreenshot(
            pngData: data,
            rect: CGRect(x: 0, y: 0, width: 64, height: 48)
        )

        let rendered = ScreenshotEditorRenderer.renderedPNGData(
            screenshot: screenshot,
            annotations: [],
            canvasSize: CGSize(width: 64, height: 48)
        )

        XCTAssertEqual(rendered, data)
    }

    func testPixelateChangesPixelsInsideRegion() throws {
        let data = try XCTUnwrap(AtlasBridge.captureRegion(x: 0, y: 0, width: 80, height: 60))
        let screenshot = CapturedScreenshot(
            pngData: data,
            rect: CGRect(x: 0, y: 0, width: 80, height: 60)
        )
        let rendered = ScreenshotEditorRenderer.renderedPNGData(
            screenshot: screenshot,
            annotations: [.pixelate(rect: CGRect(x: 20, y: 20, width: 32, height: 24))],
            canvasSize: CGSize(width: 80, height: 60)
        )

        let originalBitmap = try XCTUnwrap(NSBitmapImageRep(data: data))
        let renderedBitmap = try XCTUnwrap(NSBitmapImageRep(data: rendered))
        let originalColor = try XCTUnwrap(originalBitmap.colorAt(x: 21, y: 21)?.usingColorSpace(.deviceRGB))
        let renderedColor = try XCTUnwrap(renderedBitmap.colorAt(x: 21, y: 21)?.usingColorSpace(.deviceRGB))

        XCTAssertNotEqual(rendered, data)
        XCTAssertNotEqual(originalColor.redComponent, renderedColor.redComponent, accuracy: 0.001)
    }
}
