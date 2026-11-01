import XCTest

final class BugReportTests: BashoCase {
    private let left = CGPoint(x: 150, y: 460)
    private let right = CGPoint(x: 280, y: 460)

    private var page: XCUIElement { app.staticTexts["Bug reports"].firstMatch }

    private func saveFromMenu() {
        openMenu()
        app.buttons["Report a bug"].firstMatch.tap()
        XCTAssertTrue(page.waitForExistence(timeout: 3), "the report page opens")
    }

    func testATwoFingerHoldOnTodaySavesAReport() {
        todayTab.tap()
        settle()
        holdTwoFingers(at: left, right, for: 1.3)
        XCTAssertTrue(page.waitForExistence(timeout: 4), "the report page opens")
        XCTAssertTrue(element(label: "Report 1").exists)
        XCTAssertTrue(element(label: "Screen: today").exists)
        shot("report-two-fingers")
    }

    func testATwoFingerHoldOnTheMapSavesAReport() {
        mapTab.tap()
        settle(2)
        holdTwoFingers(at: left, right, for: 1.3)
        XCTAssertTrue(page.waitForExistence(timeout: 4), "the report page opens")
        XCTAssertTrue(element(label: "Screen: map").exists)
    }

    func testAShortTwoFingerTouchSavesNothing() {
        todayTab.tap()
        settle()
        holdTwoFingers(at: left, right, for: 0.3)
        settle(1.5)
        XCTAssertFalse(page.exists)
        XCTAssertTrue(reportFiles().isEmpty)
    }

    func testTheMenuButtonSavesAReport() {
        saveFromMenu()
        XCTAssertTrue(element(label: "Report 1").exists)
        shot("report-menu")
    }

    func testAReportIsStoredAndListedAfterARelaunch() {
        saveFromMenu()
        saveFromMenu()
        XCTAssertEqual(reportFiles().count, 2, "one file per report")
        relaunch()
        openMenu()
        let open = button(beginning: "Reports (2)")
        XCTAssertTrue(open.exists, "the count came back")
        open.tap()
        XCTAssertTrue(element(label: "Report 1").waitForExistence(timeout: 3))
        XCTAssertTrue(element(label: "Report 2").exists)
    }

    func testExportWritesEveryReportToDocuments() {
        saveFromMenu()
        XCTAssertFalse(FileManager.default.fileExists(atPath: documents.appendingPathComponent("bug-reports.toml").path))
        app.buttons["Export to Files"].firstMatch.tap()
        settle()
        shot("report-exported")
        let text = read(documents.appendingPathComponent("bug-reports.toml"))
        XCTAssertTrue(text.contains("screen = \"today\""), "the export holds the report: \(text)")
    }

    func testAReportWithAFixShowsItsPosition() {
        moveTo(latitude: 39.21, longitude: 139.91)
        mapTab.tap()
        settle(5)
        saveFromMenu()
        shot("report-position")
        let position = element(beginning: "Position: 39.21000, 139.91000")
        XCTAssertTrue(position.waitForExistence(timeout: 2), "the fix is in the report")
    }

    func testANoteIsKeptAndExported() {
        saveFromMenu()
        let note = app.textViews.firstMatch
        XCTAssertTrue(note.waitForExistence(timeout: 3))
        note.tap()
        note.typeText("The map froze")
        settle()
        shot("report-note")
        XCTAssertTrue(reportFiles().contains { read(appFolder.appendingPathComponent("reports/\($0)")).contains("The map froze") }, "the note is on disk")
        relaunch()
        openMenu()
        button(beginning: "Reports (1)").tap()
        XCTAssertTrue(app.textViews.firstMatch.waitForExistence(timeout: 3))
        XCTAssertEqual(app.textViews.firstMatch.value as? String, "The map froze")
        app.buttons["Export to Files"].firstMatch.tap()
        settle()
        XCTAssertTrue(read(documents.appendingPathComponent("bug-reports.toml")).contains("The map froze"))
    }
}
