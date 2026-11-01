import XCTest

final class NavigationTests: BashoCase {
    private func any(_ prefix: String) -> XCUIElement {
        app.descendants(matching: .any).matching(NSPredicate(format: "label BEGINSWITH %@", prefix)).firstMatch
    }

    private func containing(_ text: String) -> XCUIElement {
        app.descendants(matching: .any).matching(NSPredicate(format: "label CONTAINS %@", text)).firstMatch
    }

    func testATabTapJustAfterLaunchIsNotLost() {
        (0..<6).forEach { attempt in
            app.terminate()
            app.launch()
            XCTAssertTrue(todayTab.waitForExistence(timeout: 20))
            Thread.sleep(forTimeInterval: Double(attempt) * 0.2)
            todayTab.tap()
            XCTAssertTrue(app.buttons["Days"].firstMatch.waitForExistence(timeout: 4), "attempt \(attempt)")
        }
    }

    func testOnlyMapAndTodayAndNothingOpensByItself() {
        XCTAssertTrue(todayTab.exists)
        XCTAssertFalse(app.buttons["Places"].exists, "the Places tab is gone")
        XCTAssertFalse(app.buttons["Close"].exists, "nothing opened by itself")
        shot("launch")
    }

    func testTodayListsTheDayAndStepsOpenInPlace() {
        todayTab.tap()
        let header = any("Day ")
        XCTAssertTrue(header.waitForExistence(timeout: 3))
        XCTAssertTrue(containing("GOAL 1").waitForExistence(timeout: 3), "goals head the list")
        shot("today")
        let next = app.buttons.matching(NSPredicate(format: "label BEGINSWITH '‹ Day'")).firstMatch
        XCTAssertTrue(next.exists, "another day's button")
        next.tap()
        XCTAssertTrue(containing("GOAL 1").waitForExistence(timeout: 3))
        shot("today-next")
        let step = app.buttons.matching(NSPredicate(format: "label CONTAINS 'Get '")).element(boundBy: 1)
        XCTAssertTrue(step.waitForExistence(timeout: 3), "steps read Get to / Get on / Get off")
        let label = step.label
        step.tap()
        XCTAssertTrue(any("WHY").waitForExistence(timeout: 2), "a tapped step opens in place")
        shot("step-open")
        app.buttons[label].firstMatch.tap()
        mapTab.tap()
        XCTAssertTrue(todayTab.waitForExistence(timeout: 2))
        shot("map")
    }

    func testTheMapTopLineOpensTodayAtThatStep() {
        let top = app.buttons.matching(NSPredicate(format: "label BEGINSWITH '▶'")).firstMatch
        guard top.waitForExistence(timeout: 3) else {
            shot("map-no-step")
            return
        }
        top.tap()
        XCTAssertTrue(containing("GOAL").waitForExistence(timeout: 3), "Today opened")
        shot("top-to-today")
    }

    private func edge(_ across: CGFloat, _ down: CGFloat) -> XCUICoordinate {
        app.coordinate(withNormalizedOffset: CGVector(dx: 0, dy: 0)).withOffset(CGVector(dx: across, dy: down))
    }

    private func menuIsOpen() -> Bool {
        let row = app.buttons["Day 4"].firstMatch
        return row.exists && row.isHittable
    }

    func testEdgeSwipeOpensTheDayMenuAndAGoalRowJumpsToday() {
        let topLine = app.buttons.matching(NSPredicate(format: "label BEGINSWITH '▶' OR label BEGINSWITH '✓'")).firstMatch
        let mapBefore = topLine.waitForExistence(timeout: 3) ? topLine.label : ""
        edge(10, 450).press(forDuration: 0.05, thenDragTo: edge(10, 750))
        Thread.sleep(forTimeInterval: 0.5)
        XCTAssertFalse(menuIsOpen(), "a vertical drag at the edge does not open the menu")
        edge(10, 450).tap()
        Thread.sleep(forTimeInterval: 0.5)
        XCTAssertFalse(menuIsOpen(), "a tap at the edge does not open the menu")
        edge(10, 450).press(forDuration: 0.05, thenDragTo: edge(320, 455))
        XCTAssertTrue(app.buttons["Day 4"].firstMatch.waitForExistence(timeout: 3), "the edge swipe opens the menu")
        Thread.sleep(forTimeInterval: 0.5)
        shot("menu-from-map")
        let goal = app.buttons["Day 4 goal 2"].firstMatch
        XCTAssertTrue(goal.waitForExistence(timeout: 2))
        var tries = 0
        while !goal.isHittable && tries < 6 {
            edge(200, 750).press(forDuration: 0.05, thenDragTo: edge(205, 450))
            tries += 1
        }
        goal.tap()
        XCTAssertTrue(containing("GOAL 2").waitForExistence(timeout: 3), "Today opened")
        Thread.sleep(forTimeInterval: 0.8)
        XCTAssertFalse(menuIsOpen(), "picking a row closes the menu")
        let head = containing("GOAL 2")
        XCTAssertTrue(head.isHittable && head.frame.minY < 260, "Today is scrolled to goal 2 (at \(head.frame.minY))")
        XCTAssertTrue(app.buttons["Back to now"].firstMatch.exists, "browsing another day offers Back to now")
        shot("jumped-to-goal")
        mapTab.tap()
        XCTAssertTrue(banner.waitForExistence(timeout: 3), "the Map follows the menu jump")
        XCTAssertTrue(banner.label.hasPrefix("BROWSING · Day 4 ·"), "the banner names Day 4 (\(banner.label))")
        todayTab.tap()
        app.buttons["Back to now"].firstMatch.tap()
        Thread.sleep(forTimeInterval: 0.5)
        XCTAssertFalse(app.buttons["Back to now"].firstMatch.exists, "back on the current step")
        mapTab.tap()
        if !mapBefore.isEmpty {
            XCTAssertTrue(topLine.waitForExistence(timeout: 3))
            XCTAssertEqual(topLine.label, mapBefore, "Back to now returns the Map to live guidance")
        }
        shot("back-to-now")
    }

    private var banner: XCUIElement {
        app.buttons.matching(NSPredicate(format: "label BEGINSWITH 'BROWSING'")).firstMatch
    }

    private func pickFromMenu(_ label: String) {
        todayTab.tap()
        let days = app.buttons["Days"].firstMatch
        XCTAssertTrue(days.waitForExistence(timeout: 3))
        days.tap()
        let row = app.buttons[label].firstMatch
        XCTAssertTrue(row.waitForExistence(timeout: 3), "the menu lists \(label)")
        Thread.sleep(forTimeInterval: 0.5)
        var tries = 0
        while !row.isHittable && tries < 6 {
            edge(200, 750).press(forDuration: 0.05, thenDragTo: edge(205, 450))
            tries += 1
        }
        row.tap()
        Thread.sleep(forTimeInterval: 0.8)
    }

    private func stepNumber(_ label: String) -> Int? {
        guard let range = label.range(of: "step ") else { return nil }
        return Int(label[range.upperBound...].prefix(while: { $0.isNumber }))
    }

    func testAMenuJumpMovesTheMapAndBackToNowEndsBrowsing() {
        pickFromMenu("Day 4 goal 2")
        XCTAssertTrue(containing("GOAL 2").waitForExistence(timeout: 3), "Today shows the goal")
        mapTab.tap()
        XCTAssertTrue(banner.waitForExistence(timeout: 3), "the Map shows the BROWSING banner")
        XCTAssertTrue(banner.label.hasPrefix("BROWSING · Day 4 ·"), "the banner names Day 4 (\(banner.label))")
        let before = stepNumber(banner.label)
        XCTAssertNotNil(before, "the banner names a step (\(banner.label))")
        shot("map-browsing")
        app.buttons["Next step"].firstMatch.tap()
        Thread.sleep(forTimeInterval: 0.5)
        let after = stepNumber(banner.label)
        XCTAssertEqual(after, before.map { $0 + 1 }, "› moves to the next step (\(banner.label))")
        shot("map-browsing-next")
        app.buttons["Back to now"].firstMatch.tap()
        Thread.sleep(forTimeInterval: 0.5)
        XCTAssertFalse(banner.exists, "Back to now removes the banner")
        XCTAssertTrue(app.buttons["Next step"].firstMatch.exists, "the live Map keeps its step buttons")
        shot("map-back-to-now")
    }

    func testBrowsingSurvivesGoingToTheBackground() {
        pickFromMenu("Day 4")
        mapTab.tap()
        XCTAssertTrue(banner.waitForExistence(timeout: 3), "the Map shows the BROWSING banner")
        let next = app.buttons["Next step"].firstMatch
        var tries = 0
        while stepNumber(banner.label) != 3 && tries < 6 {
            next.tap()
            Thread.sleep(forTimeInterval: 0.4)
            tries += 1
        }
        XCTAssertTrue(banner.label.hasPrefix("BROWSING · Day 4 · step 3 ·"), "browsed to Day 4 step 3 (\(banner.label))")
        XCUIDevice.shared.press(.home)
        Thread.sleep(forTimeInterval: 2)
        app.activate()
        XCTAssertTrue(banner.waitForExistence(timeout: 10), "the banner is back after the app returns")
        XCTAssertTrue(banner.label.hasPrefix("BROWSING · Day 4 · step 3 ·"), "the focus survived (\(banner.label))")
        shot("browsing-after-background")
    }

    func testDaysButtonOpensTheMenuAndOutsideTapOrLeftSwipeCloses() {
        todayTab.tap()
        let days = app.buttons["Days"].firstMatch
        XCTAssertTrue(days.waitForExistence(timeout: 3))
        days.tap()
        XCTAssertTrue(app.buttons["Day 4"].firstMatch.waitForExistence(timeout: 2))
        Thread.sleep(forTimeInterval: 0.5)
        shot("menu-from-today")
        app.coordinate(withNormalizedOffset: CGVector(dx: 0.95, dy: 0.5)).tap()
        Thread.sleep(forTimeInterval: 0.8)
        XCTAssertFalse(menuIsOpen(), "a tap outside closes the menu")
        days.tap()
        XCTAssertTrue(app.buttons["Day 4"].firstMatch.waitForExistence(timeout: 2))
        Thread.sleep(forTimeInterval: 0.5)
        edge(300, 500).press(forDuration: 0.05, thenDragTo: edge(20, 505))
        Thread.sleep(forTimeInterval: 0.8)
        XCTAssertFalse(menuIsOpen(), "a left swipe on the panel closes it")
        days.tap()
        let day = app.buttons["Day 4"].firstMatch
        XCTAssertTrue(day.waitForExistence(timeout: 2))
        Thread.sleep(forTimeInterval: 0.5)
        day.tap()
        XCTAssertTrue(containing("Day 4 ·").waitForExistence(timeout: 3), "the day row opens Today on that day")
        Thread.sleep(forTimeInterval: 0.5)
        shot("jumped-to-day")
    }

    private var dayTitle: XCUIElement {
        app.descendants(matching: .any).matching(NSPredicate(format: "label MATCHES 'Day [0-9]+ · .*'")).firstMatch
    }

    private func dayNumber() -> Int? {
        guard dayTitle.waitForExistence(timeout: 3) else { return nil }
        let rest = dayTitle.label.dropFirst("Day ".count)
        return Int(rest.prefix(while: { $0.isNumber }))
    }

    private var dayMenu: XCUIElement {
        app.buttons.matching(NSPredicate(format: "label CONTAINS ' goal '")).firstMatch
    }

    private func leaveBrowsing() {
        let back = app.buttons["Back to now"].firstMatch
        if back.exists { back.tap() }
    }

    private func swipe(from start: (CGFloat, CGFloat), to end: (CGFloat, CGFloat)) {
        edge(start.0, start.1).press(forDuration: 0.05, thenDragTo: edge(end.0, end.1))
        Thread.sleep(forTimeInterval: 0.8)
    }

    func testSwipingTheMapMovesTheFocusedStepLikeItsButtons() {
        pickFromMenu("Day 4 goal 2")
        mapTab.tap()
        XCTAssertTrue(banner.waitForExistence(timeout: 3), "the Map shows the BROWSING banner")
        let before = stepNumber(banner.label)
        XCTAssertNotNil(before, "the banner names a step (\(banner.label))")
        swipe(from: (340, 520), to: (90, 525))
        XCTAssertTrue(banner.label.hasPrefix("BROWSING · Day 4 ·"), "still Day 4 (\(banner.label))")
        XCTAssertEqual(stepNumber(banner.label), before.map { $0 + 1 }, "a left swipe shows the next step (\(banner.label))")
        shot("map-swiped-left")
        swipe(from: (90, 520), to: (340, 525))
        XCTAssertEqual(stepNumber(banner.label), before, "a right swipe goes back (\(banner.label))")
        shot("map-swiped-right")
        leaveBrowsing()
    }

    func testSwipingTodayChangesTheDayLikeItsButtons() {
        pickFromMenu("Day 4")
        XCTAssertEqual(dayNumber(), 4, "Today shows Day 4")
        swipe(from: (360, 600), to: (80, 605))
        XCTAssertEqual(dayNumber(), 5, "a left swipe shows the next day (\(dayTitle.label))")
        XCTAssertTrue(containing("GOAL 1").waitForExistence(timeout: 3), "the next day's goals")
        shot("today-swiped-left")
        swipe(from: (80, 600), to: (360, 605))
        XCTAssertEqual(dayNumber(), 4, "a right swipe goes back (\(dayTitle.label))")
        shot("today-swiped-right")
        leaveBrowsing()
    }

    func testScrollingTodayAndEdgeSwipesDoNotChangeTheDay() {
        pickFromMenu("Day 4")
        XCTAssertEqual(dayNumber(), 4, "Today shows Day 4")
        swipe(from: (200, 750), to: (230, 300))
        swipe(from: (230, 300), to: (200, 750))
        XCTAssertEqual(dayNumber(), 4, "a vertical scroll keeps the day (\(dayTitle.label))")
        swipe(from: (300, 750), to: (120, 520))
        XCTAssertEqual(dayNumber(), 4, "a diagonal drag keeps the day (\(dayTitle.label))")
        Thread.sleep(forTimeInterval: 2)
        edge(10, 500).press(forDuration: 0.05, thenDragTo: edge(320, 505))
        XCTAssertTrue(dayMenu.waitForExistence(timeout: 3), "the edge swipe opens the menu")
        Thread.sleep(forTimeInterval: 0.5)
        XCTAssertTrue(button(beginning: "Trip:").isHittable, "the menu is on screen")
        shot("today-edge-menu")
        app.coordinate(withNormalizedOffset: CGVector(dx: 0.95, dy: 0.5)).tap()
        Thread.sleep(forTimeInterval: 0.8)
        XCTAssertFalse(dayMenu.exists, "a tap outside closes the menu")
        XCTAssertEqual(dayNumber(), 4, "the edge swipe did not change the day (\(dayTitle.label))")
        leaveBrowsing()
    }
}

