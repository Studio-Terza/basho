import CoreLocation
import UIKit
import XCTest

class BashoCase: XCTestCase {
    var app: XCUIApplication!

    var arguments: [String] { [] }

    private var environment: [String: String] { ProcessInfo.processInfo.environment }
    var fixtures: URL { URL(fileURLWithPath: environment["FIXTURES"] ?? "/") }
    var appData: URL { URL(fileURLWithPath: environment["APP_DATA"] ?? "/") }
    var appFolder: URL { appData.appendingPathComponent("Library/Application Support/basho") }
    var documents: URL { appData.appendingPathComponent("Documents") }

    override func setUpWithError() throws {
        continueAfterFailure = false
        app = XCUIApplication(bundleIdentifier: "com.cinglein.basho")
        app.terminate()
        forgetEverything()
        launch()
    }

    func forgetEverything() {
        try? FileManager.default.removeItem(at: appFolder)
        try? FileManager.default.removeItem(at: documents.appendingPathComponent("bug-reports.toml"))
    }

    func launch() {
        app.launchArguments = arguments
        app.launch()
        XCTAssertTrue(todayTab.waitForExistence(timeout: 20), "the app starts")
        settle(2)
    }

    func relaunch() {
        app.terminate()
        launch()
    }

    var mapTab: XCUIElement { app.buttons["Map"].firstMatch }
    var todayTab: XCUIElement { app.buttons["Today"].firstMatch }

    func element(label text: String) -> XCUIElement {
        app.descendants(matching: .any).matching(NSPredicate(format: "label ==[c] %@", text)).firstMatch
    }

    func element(beginning text: String) -> XCUIElement {
        app.descendants(matching: .any).matching(NSPredicate(format: "label BEGINSWITH %@", text)).firstMatch
    }

    func element(containing text: String) -> XCUIElement {
        app.descendants(matching: .any).matching(NSPredicate(format: "label CONTAINS %@", text)).firstMatch
    }

    func button(beginning text: String) -> XCUIElement {
        app.buttons.matching(NSPredicate(format: "label BEGINSWITH %@", text)).firstMatch
    }

    func shot(_ name: String) {
        guard let folder = environment["SHOTS"] else { return }
        let data = XCUIScreen.main.screenshot().pngRepresentation
        try? data.write(to: URL(fileURLWithPath: folder).appendingPathComponent("\(name).png"))
    }

    func settle(_ seconds: TimeInterval = 0.8) {
        Thread.sleep(forTimeInterval: seconds)
    }

    func spot(_ across: CGFloat, _ down: CGFloat) -> XCUICoordinate {
        app.coordinate(withNormalizedOffset: .zero).withOffset(CGVector(dx: across, dy: down))
    }

    func drag(from start: (CGFloat, CGFloat), to end: (CGFloat, CGFloat)) {
        spot(start.0, start.1).press(forDuration: 0.05, thenDragTo: spot(end.0, end.1))
        settle()
    }

    func openMenu() {
        todayTab.tap()
        settle(1.5)
        let days = app.buttons["Days"].firstMatch
        XCTAssertTrue(days.waitForExistence(timeout: 5), "Today shows the menu button")
        days.tap()
        XCTAssertTrue(button(beginning: "Trip:").waitForExistence(timeout: 3), "the menu opens")
        settle()
    }

    func holdTwoFingers(at first: CGPoint, _ second: CGPoint, for seconds: Double) {
        TouchRecord.named("two fingers").holding([first, second], for: seconds).play()
    }

    func moveTo(latitude: Double, longitude: Double) {
        XCUIDevice.shared.location = XCUILocation(
            location: CLLocation(latitude: latitude, longitude: longitude))
    }

    func read(_ file: URL) -> String {
        (try? String(contentsOf: file, encoding: .utf8)) ?? ""
    }

    func reportFiles() -> [String] {
        let folder = appFolder.appendingPathComponent("reports")
        return (try? FileManager.default.contentsOfDirectory(atPath: folder.path)) ?? []
    }

    func eventually(_ what: String, within seconds: TimeInterval = 8, _ holds: () -> Bool) {
        let end = Date().addingTimeInterval(seconds)
        while Date() < end {
            if holds() { return }
            settle(0.25)
        }
        XCTAssertTrue(holds(), what)
    }
}

extension BashoCase {
    func tree() -> XCUIElementSnapshot? {
        try? app.snapshot()
    }

    func flattened(_ node: XCUIElementSnapshot?) -> [XCUIElementSnapshot] {
        guard let node else { return [] }
        return [node] + node.children.flatMap { flattened($0) }
    }

    func onScreen(_ type: XCUIElement.ElementType) -> [XCUIElementSnapshot] {
        flattened(tree()).filter { $0.elementType == type && !$0.label.isEmpty && $0.frame.minY > 200 && $0.frame.maxY < 820 && $0.frame.minY < $0.frame.maxY }
    }

    func pageWords() -> [String] {
        Array(flattened(tree()).filter { $0.elementType == .staticText }.map(\.label).prefix(10))
    }
}

extension BashoCase {
    func tripStartingNow() -> String {
        let now = Date()
        let calendar = Calendar(identifier: .gregorian)
        var utc = calendar
        utc.timeZone = TimeZone(secondsFromGMT: 0)!
        let parts = utc.dateComponents([.hour, .minute], from: now)
        let past = (parts.hour ?? 0) * 60 + (parts.minute ?? 0)
        let wanted = 9 * 60
        let raw = ((wanted - past) % 1440 + 1440) % 1440
        let offset = raw > 840 ? raw - 1440 : raw
        let sign = offset < 0 ? "-" : "+"
        let zone = String(format: "%@%02d:%02d", sign, abs(offset) / 60, abs(offset) % 60)
        let formatter = DateFormatter()
        formatter.locale = Locale(identifier: "en_US_POSIX")
        formatter.dateFormat = "yyyy-MM-dd"
        formatter.timeZone = TimeZone(secondsFromGMT: offset * 60)
        let text = read(fixtures.appendingPathComponent("sample.toml"))
            .replacingOccurrences(of: "\"-04:00\"", with: "\"\(zone)\"")
            .replacingOccurrences(of: "2027-05-03", with: formatter.string(from: now))
            .replacingOccurrences(of: "2027-05-04", with: formatter.string(from: now.addingTimeInterval(86400)))
        let file = appData.appendingPathComponent("tmp/today-trip.toml")
        try? text.write(to: file, atomically: true, encoding: .utf8)
        return file.path
    }
}
