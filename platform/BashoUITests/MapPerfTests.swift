import XCTest

final class MapPerfTests: BashoCase {
    override var arguments: [String] { [tripStartingNow()] }

    private var seconds: TimeInterval { TimeInterval(ProcessInfo.processInfo.environment["PHASE_SECONDS"] ?? "") ?? 10 }

    private func phase(_ name: String, _ work: () -> Void) {
        print("PHASE START \(name)")
        let end = Date().addingTimeInterval(seconds)
        repeat { work() } while Date() < end
        print("PHASE END \(name)")
    }

    func testTheMapRunsForAPhaseOfEachKind() {
        mapTab.tap()
        settle(5)
        phase("idle") { settle(1) }
        app.buttons["Explore"].firstMatch.tap()
        settle(2)
        var east = true
        phase("pan") {
            drag(from: east ? (340, 500) : (90, 500), to: east ? (90, 505) : (340, 505))
            east.toggle()
        }
        app.buttons["Guide"].firstMatch.tap()
        settle(2)
        var step = 0.0
        phase("fixes") {
            step += 1
            moveTo(latitude: 38.0, longitude: -77.998 + step * 0.0002)
            settle(3)
        }
    }
}
