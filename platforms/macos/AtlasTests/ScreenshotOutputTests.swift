import XCTest
@testable import Atlas

final class ScreenshotOutputTests: XCTestCase {
    func testPngFilenameUsesTimestamp() {
        let date = Date(timeIntervalSince1970: 1_704_067_200)
        let filename = ScreenshotOutput.filename(for: date)
        XCTAssertEqual(filename, "Atlas Screenshot 2024-01-01 00.00.00.png")
    }

    func testWritePngData() throws {
        let data = Data([0x89, 0x50, 0x4E, 0x47])
        let directory = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString, isDirectory: true)
        try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: true)

        let url = try ScreenshotOutput.writePNG(data, to: directory, date: Date(timeIntervalSince1970: 1_704_067_200))

        XCTAssertEqual(url.lastPathComponent, "Atlas Screenshot 2024-01-01 00.00.00.png")
        XCTAssertEqual(try Data(contentsOf: url), data)
    }
}
