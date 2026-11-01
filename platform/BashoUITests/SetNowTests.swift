import XCTest

final class SetNowTests: BashoCase {
    override var arguments: [String] { [tripStartingNow()] }

    private var done: XCUIElement { app.buttons["Done ›"].firstMatch }
    private var here: XCUIElement { app.buttons["I'm here"].firstMatch }
    private var first: XCUIElement { button(beginning: "1 Get") }
    private var second: XCUIElement { button(beginning: "2 Get") }

    private func onToday() {
        todayTab.tap()
        XCTAssertTrue(first.waitForExistence(timeout: 5), "Today lists the steps")
        settle()
    }

    private func reachAndTap(_ target: XCUIElement) {
        var tries = 0
        while !target.isHittable && tries < 6 {
            drag(from: (200, 700), to: (205, 400))
            tries += 1
        }
        shot("set-now-reached")
        target.tap()
    }

    private func expectCurrent(_ step: XCUIElement, _ next: XCUIElement?, _ why: String) {
        XCTAssertTrue(done.waitForExistence(timeout: 3), "Done shows: \(why)")
        XCTAssertGreaterThan(done.frame.minY, step.frame.maxY - 1, "Done sits under the current step: \(why)")
        if let next { XCTAssertLessThan(done.frame.maxY, next.frame.minY + 1, why) }
    }

    func testDoneMovesToTheNextStep() {
        onToday()
        shot("set-now-start")
        expectCurrent(first, second, "the walk starts on step 1")
        done.tap()
        settle()
        shot("set-now-done")
        expectCurrent(second, nil, "Done moved on")
    }

    func testIAmHereMovesForwardToALaterStep() {
        onToday()
        second.tap()
        settle()
        XCTAssertTrue(here.waitForExistence(timeout: 3), "a later step has I'm here")
        shot("set-now-later")
        reachAndTap(here)
        settle()
        expectCurrent(second, nil, "I'm here moved forward")
    }

    func testIAmHereGoesBackOnlyWhenAsked() {
        onToday()
        done.tap()
        settle()
        expectCurrent(second, nil, "Done moved on")
        first.tap()
        settle()
        XCTAssertTrue(here.waitForExistence(timeout: 3), "a finished step has I'm here")
        here.tap()
        settle()
        shot("set-now-back")
        expectCurrent(first, second, "I'm here went back")
    }

    func testIAmHereWorksWhileBrowsingFromTheMap() {
        mapTab.tap()
        settle(2)
        app.buttons["Next step"].firstMatch.tap()
        settle()
        XCTAssertTrue(button(beginning: "BROWSING").waitForExistence(timeout: 3), "browsing step 2")
        shot("set-now-browsing")
        onToday()
        second.tap()
        settle()
        XCTAssertTrue(here.waitForExistence(timeout: 3), "the browsed step has I'm here")
        reachAndTap(here)
        settle()
        expectCurrent(second, nil, "I'm here while browsing")
        mapTab.tap()
        settle()
        XCTAssertFalse(button(beginning: "BROWSING").exists, "browsing ended")
    }

    func testTheTrackerDoesNotUndoAManualSet() {
        moveTo(latitude: 38.0, longitude: -77.9995)
        mapTab.tap()
        settle(5)
        onToday()
        done.tap()
        settle()
        expectCurrent(second, nil, "Done moved on")
        moveTo(latitude: 38.0, longitude: -77.999)
        settle(6)
        moveTo(latitude: 38.0, longitude: -77.9985)
        settle(6)
        shot("set-now-held")
        expectCurrent(second, nil, "fixes on step 1 did not move it back")
    }

    func testTheTrackerStillMovesForwardAlone() {
        moveTo(latitude: 38.0, longitude: -77.9966)
        mapTab.tap()
        settle(5)
        onToday()
        expectCurrent(first, second, "the fix is on step 1")
        moveTo(latitude: 38.0, longitude: -77.9958)
        settle(5)
        moveTo(latitude: 38.0, longitude: -77.9950)
        shot("set-now-tracker-walked")
        eventually("the tracker reached step 2", within: 10) {
            done.exists && done.frame.minY > second.frame.maxY - 1
        }
        shot("set-now-tracker-moved")
    }

    func testASetStepSurvivesARelaunch() {
        onToday()
        done.tap()
        settle(2)
        relaunch()
        onToday()
        expectCurrent(second, nil, "the set step came back")
    }
}
