import UIKit
import XCTest

final class MapTests: BashoCase {
    private var slot: XCUIElement { app.images["Map of this step, with you and the place at its end"].firstMatch }
    private var level: String { ["Leg", "Goal", "Whole day"].first { app.staticTexts[$0].firstMatch.exists } ?? "none" }
    private var topLine: String { button(beginning: "▶").exists ? button(beginning: "▶").label : app.buttons.element(boundBy: 1).label }

    override func setUpWithError() throws {
        try super.setUpWithError()
        mapTab.tap()
        XCTAssertTrue(slot.waitForExistence(timeout: 8))
        settle(3)
    }

    private func picture() -> UIImage { slot.screenshot().image }

    private func difference(_ first: UIImage, _ second: UIImage) -> Double {
        let side = 96
        func pixels(_ image: UIImage) -> [UInt8] {
            var bytes = [UInt8](repeating: 0, count: side * side)
            let space = CGColorSpaceCreateDeviceGray()
            bytes.withUnsafeMutableBytes { raw in
                let context = CGContext(data: raw.baseAddress, width: side, height: side, bitsPerComponent: 8, bytesPerRow: side, space: space, bitmapInfo: CGImageAlphaInfo.none.rawValue)
                context?.draw(image.cgImage!, in: CGRect(x: 0, y: 0, width: side, height: side))
            }
            return bytes
        }
        let total = zip(pixels(first), pixels(second)).reduce(0) { $0 + abs(Int($1.0) - Int($1.1)) }
        return Double(total) / Double(side * side)
    }

    private func pickedShown() -> Bool {
        let top = button(beginning: "▶").frame.maxY
        return flattened(tree()).contains { $0.elementType == .staticText && $0.frame.minY >= top && $0.frame.maxY <= slot.frame.minY }
    }

    private func pinch(from start: CGFloat, to end: CGFloat, for seconds: Double = 0.4) {
        let middle = CGPoint(x: slot.frame.midX, y: slot.frame.midY)
        func finger(_ spread: CGFloat, _ side: CGFloat) -> CGPoint { CGPoint(x: middle.x + side * spread / 2, y: middle.y) }
        TouchRecord.named("pinch")
            .moving(from: finger(start, -1), to: finger(end, -1), for: seconds)
            .moving(from: finger(start, 1), to: finger(end, 1), for: seconds)
            .play()
    }

    private func pinchTogether() { pinch(from: 260, to: 90) }
    private func pinchApart() { pinch(from: 90, to: 260) }

    private func expectLevel(_ name: String) {
        settle(0.5)
        shot("level-\(name)")
        eventually("the level is \(name), not \(level)", within: 3) { level == name }
    }

    func testPinchChangesTheLevel() {
        expectLevel("Leg")
        let leg = picture()
        pinchTogether()
        settle(1)
        shot("pinch-1")
        expectLevel("Goal")
        settle(1.5)
        shot("map-goal")
        pinchTogether()
        expectLevel("Whole day")
        settle(1.5)
        shot("map-day")
        pinchApart()
        expectLevel("Goal")
        pinchApart()
        expectLevel("Leg")
        settle(1.5)
        XCTAssertLessThan(difference(leg, picture()), 3, "back on the same frame")
        XCTAssertTrue(reportFiles().isEmpty, "a pinch is not a bug report")
    }

    func testASlowPinchIsNotABugReport() {
        pinch(from: 260, to: 90, for: 1.5)
        settle(1.5)
        XCTAssertFalse(app.staticTexts["Bug reports"].firstMatch.exists, "a slow pinch stays on the map")
        XCTAssertTrue(reportFiles().isEmpty)
        expectLevel("Goal")
    }

    func testAPinchWithOneFingerStillIsNotABugReport() {
        TouchRecord.named("one finger still")
            .holding([CGPoint(x: 200, y: 460)], for: 1.5)
            .moving(from: CGPoint(x: 240, y: 460), to: CGPoint(x: 380, y: 460), for: 1.5)
            .play()
        settle(1.5)
        XCTAssertFalse(app.staticTexts["Bug reports"].firstMatch.exists, "a pinch stays on the map")
        expectLevel("Leg")
    }

    func testASlowPinchApartIsNotABugReport() {
        pinch(from: 90, to: 260, for: 1.5)
        settle(1.5)
        XCTAssertFalse(app.staticTexts["Bug reports"].firstMatch.exists, "a slow pinch stays on the map")
        XCTAssertTrue(reportFiles().isEmpty)
    }

    func testTappingAMarkNamesIt() {
        var found = false
        let frame = slot.frame
        for down in stride(from: frame.minY + 60, to: frame.maxY - 60, by: 44) where !found {
            for across in stride(from: frame.minX + 40, to: frame.maxX - 30, by: 44) where !found {
                spot(across, down).tap()
                settle(0.25)
                found = pickedShown()
            }
        }
        shot("map-tapped")
        XCTAssertTrue(found, "a mark's kind and name show under the top row")
    }

    func testTheKeyListsWhatIsDrawn() {
        app.buttons["Key"].firstMatch.tap()
        settle()
        shot("map-key")
        XCTAssertTrue(app.staticTexts.allElementsBoundByIndex.contains { $0.label.lowercased().contains("road") || $0.label.lowercased().contains("street") }, "the key names roads")
    }

    func testGuideSwipesStepAndExploreSwipesPan() {
        let first = app.buttons["Next step"].firstMatch
        XCTAssertTrue(first.exists)
        let before = topLine
        drag(from: (340, 500), to: (90, 505))
        settle()
        XCTAssertNotEqual(topLine, before, "in Guide a swipe steps")
        drag(from: (90, 500), to: (340, 505))
        settle()
        XCTAssertEqual(topLine, before, "and steps back")

        let guideFrame = picture()
        app.buttons["Explore"].firstMatch.tap()
        settle()
        XCTAssertTrue(app.buttons["Guide"].firstMatch.waitForExistence(timeout: 2))
        XCTAssertEqual(level, "none", "Explore shows no level")
        drag(from: (340, 500), to: (120, 505))
        settle(2)
        shot("map-explore-panned")
        XCTAssertEqual(topLine, before, "in Explore a swipe leaves the step alone")
        XCTAssertGreaterThan(difference(guideFrame, picture()), 3, "the map panned")
        let panned = picture()
        pinchTogether()
        settle(2)
        XCTAssertGreaterThan(difference(panned, picture()), 3, "the map zoomed")
        XCTAssertFalse(app.staticTexts["Bug reports"].firstMatch.exists)
        app.buttons["Guide"].firstMatch.tap()
        settle(2)
        XCTAssertTrue(app.buttons["Explore"].firstMatch.exists)
        expectLevel("Leg")
        XCTAssertLessThan(difference(guideFrame, picture()), 3, "Guide refits")
        shot("map-guide-again")
    }

    func testTheMapKeepsItsPlaceAcrossTabs() {
        let tabEnd = mapTab.frame.maxY
        XCTAssertLessThanOrEqual(tabEnd, 898, "the tab bar clears the home indicator")
        shot("tabs-before")
        let before = picture()
        todayTab.tap()
        settle()
        mapTab.tap()
        settle(2)
        shot("tabs-after")
        XCTAssertLessThan(difference(before, picture()), 3)
        XCTAssertEqual(mapTab.frame.maxY, tabEnd, accuracy: 1, "the tab bar did not move")
    }
}
