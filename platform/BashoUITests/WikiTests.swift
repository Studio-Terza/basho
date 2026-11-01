import XCTest

final class WikiTests: BashoCase {
    private var back: XCUIElement { button(beginning: "‹ ") }
    private let entry = "Eastfield"

    private func openIndex() {
        openMenu()
        app.buttons["Index"].firstMatch.tap()
        XCTAssertTrue(back.waitForExistence(timeout: 3), "the Index opens")
        settle()
    }

    private func openEntry() {
        todayTab.tap()
        let link = app.links[entry].firstMatch
        XCTAssertTrue(link.waitForExistence(timeout: 5), "Today links \(entry)")
        link.tap()
        XCTAssertTrue(back.waitForExistence(timeout: 3), "the entry opens")
        settle()
    }

    private func wordTop(_ word: String) -> CGFloat {
        flattened(tree()).first { $0.elementType == .staticText && $0.label == word }?.frame.minY ?? -1
    }

    func testBackRestoresTheExactScroll() throws {
        openIndex()
        drag(from: (200, 760), to: (205, 240))
        drag(from: (200, 760), to: (205, 240))
        settle(2)
        let link = try XCTUnwrap(onScreen(.link).first, "a link on screen")
        let place = link.frame
        shot("wiki-before")
        app.links[link.label].firstMatch.tap()
        settle()
        let page = pageWords()
        shot("wiki-entry")
        drag(from: (200, 760), to: (205, 300))
        back.tap()
        settle()
        shot("wiki-restored")
        XCTAssertNotEqual(pageWords(), page, "back on the Index")
        let again = app.links[link.label].firstMatch
        XCTAssertTrue(again.waitForExistence(timeout: 3))
        XCTAssertEqual(again.frame.minY, place.minY, accuracy: 2, "the same scroll")
    }

    func testBackToTodayRestoresItsScroll() throws {
        todayTab.tap()
        settle(2)
        drag(from: (200, 600), to: (205, 520))
        settle(2)
        let link = try XCTUnwrap(onScreen(.link).first { $0.label.hasPrefix("鳥海山") }, "a place link on screen")
        let place = link.frame
        spot(place.midX, place.midY).tap()
        let today = button(beginning: "‹ Today")
        XCTAssertTrue(today.waitForExistence(timeout: 3), "the place opens")
        settle()
        today.tap()
        settle()
        shot("wiki-today-restored")
        let found = onScreen(.link).first { $0.label == link.label }
        XCTAssertEqual(found?.frame.minY ?? 0, place.minY, accuracy: 2, "the same scroll")
    }

    func testForwardRestoresTheEntryScroll() throws {
        openIndex()
        let link = try XCTUnwrap(onScreen(.link).first)
        app.links[link.label].firstMatch.tap()
        settle()
        drag(from: (200, 760), to: (205, 500))
        settle(2)
        let word = try XCTUnwrap(onScreen(.staticText).first).label
        let top = wordTop(word)
        drag(from: (120, 520), to: (360, 525))
        XCTAssertNotEqual(wordTop(word), top, "a right swipe goes back")
        drag(from: (360, 520), to: (120, 525))
        settle()
        XCTAssertEqual(wordTop(word), top, accuracy: 2, "a left swipe comes back to the same scroll")
    }

    func testTheSectionBarJumps() throws {
        openEntry()
        shot("wiki-section-bar")
        let bar = app.buttons["Geography"].firstMatch
        XCTAssertTrue(bar.waitForExistence(timeout: 3), "a page with sections")
                let name = bar.label
        bar.tap()
        settle()
        shot("wiki-section-jumped")
        XCTAssertTrue(bar.isHittable, "the bar stays on screen")
        let barEnd = app.buttons["Linked here"].firstMatch.frame.maxY
        let head = app.staticTexts.matching(NSPredicate(format: "label ==[c] %@", name))
        let heading = try XCTUnwrap((0..<head.count).map { head.element(boundBy: $0) }.last)
        XCTAssertGreaterThanOrEqual(heading.frame.minY, barEnd - 2, "the heading is not under the bar")
        XCTAssertLessThan(heading.frame.minY, barEnd + 90, "the heading is just under the bar")
    }

    func testSwipesTurnThePage() {
        openIndex()
        let index = pageWords()
        let link = onScreen(.link).first
        app.links[link?.label ?? ""].firstMatch.tap()
        settle()
        let page = pageWords()
        XCTAssertNotEqual(page, index)
        drag(from: (120, 520), to: (360, 525))
        XCTAssertEqual(pageWords(), index, "a right swipe goes back")
        shot("wiki-swiped-back")
        drag(from: (360, 520), to: (120, 525))
        XCTAssertEqual(pageWords(), page, "a left swipe goes forward")
        shot("wiki-swiped-forward")
    }

    func testBackLeavesTheWikiForTheScreenItCameFrom() {
        openIndex()
        back.tap()
        settle()
        XCTAssertTrue(app.buttons["Days"].firstMatch.waitForExistence(timeout: 3), "Today again")
    }
}
