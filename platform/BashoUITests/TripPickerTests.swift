import XCTest

final class TripPickerTests: BashoCase {
    private func openPicker() {
        openMenu()
        button(beginning: "Trip:").tap()
        XCTAssertTrue(app.buttons["Use the built-in trip"].waitForExistence(timeout: 3), "the picker page opens")
    }

    private func pick(_ file: String) {
        app.buttons["Open a trip file"].firstMatch.tap()
        let browse = app.tabBars.buttons["Browse"].firstMatch
        XCTAssertTrue(browse.waitForExistence(timeout: 8), "the phone's own picker opens")
        browse.tap()
        settle()
        shot("picker-browse")
        let phone = element(label: "On My iPhone")
        if phone.waitForExistence(timeout: 3) { phone.tap() }
        settle()
        shot("picker-phone")
        let folder = app.cells.matching(NSPredicate(format: "label BEGINSWITH %@", "Basho")).firstMatch
        XCTAssertTrue(folder.waitForExistence(timeout: 5), "the app's folder is listed")
        folder.tap()
        settle()
        shot("picker-folder-\(file)")
        let row = app.cells.matching(NSPredicate(format: "label CONTAINS %@", file)).firstMatch
        XCTAssertTrue(row.waitForExistence(timeout: 5), "\(file) is listed")
        row.tap()
        settle(2)
    }

    private func currentTripName() -> String {
        openMenu()
        let label = button(beginning: "Trip:").label
        app.coordinate(withNormalizedOffset: CGVector(dx: 0.95, dy: 0.5)).tap()
        settle()
        return label
    }

    func testThePickerOpensFromTheMenu() {
        openPicker()
        XCTAssertTrue(app.buttons["Open a trip file"].firstMatch.exists)
        shot("picker-page")
    }

    func testATomlFileCanBeChosenAndIsRead() {
        openPicker()
        pick("sample-trip")
        shot("picker-chosen")
        XCTAssertFalse(element(containing: "did not open").exists, "no refusal")
        XCTAssertEqual(currentTripName(), "Trip: Sample")
    }

    func testTheChosenTripSurvivesARelaunch() {
        openPicker()
        pick("sample-trip")
        XCTAssertEqual(currentTripName(), "Trip: Sample")
        relaunch()
        XCTAssertEqual(currentTripName(), "Trip: Sample", "the chosen trip came back")
        XCTAssertTrue(FileManager.default.fileExists(atPath: appFolder.appendingPathComponent("chosen-trip.toml").path))
    }

    func testABadFileShowsWhyAndChangesNothing() {
        let before = currentTripName()
        openPicker()
        pick("bad-trip")
        shot("picker-bad-file")
        let refusal = element(containing: "did not open")
        XCTAssertTrue(refusal.waitForExistence(timeout: 3), "the page says why")
        XCTAssertTrue(element(containing: "The current trip did not change").exists)
        XCTAssertTrue(refusal.isHittable, "the reason is on screen")
        app.buttons["‹ Back"].firstMatch.tap()
        XCTAssertEqual(currentTripName(), before)
        XCTAssertFalse(FileManager.default.fileExists(atPath: appFolder.appendingPathComponent("chosen-trip.toml").path))
    }

    func testAFileThatIsNotTextIsRefused() {
        openPicker()
        pick("not-text")
        shot("picker-not-text")
        XCTAssertTrue(element(containing: "not plain text").waitForExistence(timeout: 3))
    }

    func testALongTimeLeavesTheStepTitleReadable() {
        moveTo(latitude: 38.0, longitude: -77.992)
        openPicker()
        pick("sample-trip")
        todayTab.tap()
        let head = button(beginning: "2 Get")
        XCTAssertTrue(head.waitForExistence(timeout: 5))
        settle(4)
        shot("picker-long-time")
        XCTAssertLessThan(head.frame.height, 120, "the title stays on a line or two")
        XCTAssertLessThanOrEqual(head.frame.maxX, 415, "the row stays inside the page")
    }

    func testTheBuiltInTripComesBack() {
        openPicker()
        pick("sample-trip")
        XCTAssertEqual(currentTripName(), "Trip: Sample")
        openPicker()
        app.buttons["Use the built-in trip"].firstMatch.tap()
        settle()
        XCTAssertEqual(currentTripName(), "Trip: Sample")
    }
}
