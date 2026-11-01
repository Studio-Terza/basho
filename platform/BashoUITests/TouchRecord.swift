import UIKit
import XCTest

struct TouchRecord {
    private typealias PathStart = @convention(c) (AnyObject, Selector, CGPoint, Double) -> AnyObject
    private typealias PathMove = @convention(c) (AnyObject, Selector, CGPoint, Double) -> Void
    private typealias PathLift = @convention(c) (AnyObject, Selector, Double) -> Void
    private typealias RecordStart = @convention(c) (AnyObject, Selector, NSString, Int) -> AnyObject
    private typealias RecordAdd = @convention(c) (AnyObject, Selector, AnyObject) -> Void
    private typealias RecordPlay = @convention(c) (AnyObject, Selector, UnsafeMutablePointer<NSError?>?) -> Bool

    private let record: NSObject

    private static func fresh(_ name: String) -> AnyObject {
        let kind = NSClassFromString(name) as! NSObject.Type
        return kind.perform(NSSelectorFromString("alloc")).takeUnretainedValue()
    }

    private static func function<Shape>(_ target: AnyObject, _ name: String, as shape: Shape.Type) -> Shape {
        let found = (target as! NSObject).method(for: NSSelectorFromString(name))
        return unsafeBitCast(found, to: shape)
    }

    static func named(_ name: String) -> TouchRecord {
        let created = fresh("XCSynthesizedEventRecord")
        let start = function(created, "initWithName:interfaceOrientation:", as: RecordStart.self)
        let portrait = UIInterfaceOrientation.portrait.rawValue
        return TouchRecord(record: start(created, NSSelectorFromString("initWithName:interfaceOrientation:"), name as NSString, portrait) as! NSObject)
    }

    func moving(from start: CGPoint, to end: CGPoint, for seconds: Double) -> TouchRecord {
        let created = Self.fresh("XCPointerEventPath")
        let begin = Self.function(created, "initForTouchAtPoint:offset:", as: PathStart.self)
        let path = begin(created, NSSelectorFromString("initForTouchAtPoint:offset:"), start, 0)
        let move = Self.function(path, "moveToPoint:atOffset:", as: PathMove.self)
        move(path, NSSelectorFromString("moveToPoint:atOffset:"), end, seconds - 0.1)
        let lift = Self.function(path, "liftUpAtOffset:", as: PathLift.self)
        lift(path, NSSelectorFromString("liftUpAtOffset:"), seconds)
        let add = Self.function(record, "addPointerEventPath:", as: RecordAdd.self)
        add(record, NSSelectorFromString("addPointerEventPath:"), path)
        return self
    }

    func holding(_ points: [CGPoint], for seconds: Double) -> TouchRecord {
        points.forEach { point in
            let created = Self.fresh("XCPointerEventPath")
            let start = Self.function(created, "initForTouchAtPoint:offset:", as: PathStart.self)
            let path = start(created, NSSelectorFromString("initForTouchAtPoint:offset:"), point, 0)
            let lift = Self.function(path, "liftUpAtOffset:", as: PathLift.self)
            lift(path, NSSelectorFromString("liftUpAtOffset:"), seconds)
            let add = Self.function(record, "addPointerEventPath:", as: RecordAdd.self)
            add(record, NSSelectorFromString("addPointerEventPath:"), path)
        }
        return self
    }

    func play() {
        var problem: NSError?
        let play = Self.function(record, "synthesizeWithError:", as: RecordPlay.self)
        XCTAssertTrue(play(record, NSSelectorFromString("synthesizeWithError:"), &problem), "\(String(describing: problem))")
    }
}
