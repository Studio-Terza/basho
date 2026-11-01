use std::rc::Rc;
use std::time::{Duration, UNIX_EPOCH};

use dioxus::prelude::*;
use guide::clock::TripTime;
use guide::geo::{Bearing, Coordinate, Degrees, Distance};
use guide::guidance::Sighting;
use objc2::rc::Retained;
use objc2_core_location::{
    CLAccuracyAuthorization, CLActivityType, CLAuthorizationStatus, CLLocationManager,
    kCLLocationAccuracyBest, kCLLocationAccuracyHundredMeters,
};

use layout::measure::Ground;

use crate::app::screen::Screen;
use crate::app::session::Session;
use crate::app::session::location_access::LocationAccess;
use crate::app::session::measure::{Angle, Cadence};
use crate::app::session::wall_clock::WallClock;

pub struct LocationFeed {
    manager: Retained<CLLocationManager>,
    clock: WallClock,
}

impl LocationFeed {
    pub fn follow(session: &Session) {
        let position = session.position;
        let spread = session.spread;
        let heading = session.heading;
        let access = session.access;
        let screen = session.screen;
        let wakes = session.wakes;
        let clock = session.clock;
        let feed = use_hook(|| {
            let feed = Rc::new(LocationFeed::start(clock));
            Self::poll(Rc::clone(&feed), position, spread, access);
            Self::turn(Rc::clone(&feed), heading);
            feed
        });
        let woken = Rc::clone(&feed);
        let easing = Rc::clone(&feed);
        let tracked = session.clone();
        use_effect(move || easing.ease(tracked.halted().is_some()));
        use_effect(move || feed.aim_compass(&screen.read()));
        use_effect(move || {
            wakes.read().ever().then(|| woken.restart());
        });
    }

    pub fn aim_compass(&self, screen: &Screen) {
        if *screen == Screen::Map {
            unsafe { self.manager.startUpdatingHeading() };
        } else {
            unsafe { self.manager.stopUpdatingHeading() };
        }
    }

    pub fn ease(&self, halted: bool) {
        let accuracy = if halted {
            unsafe { kCLLocationAccuracyHundredMeters }
        } else {
            unsafe { kCLLocationAccuracyBest }
        };
        unsafe { self.manager.setDesiredAccuracy(accuracy) };
    }

    pub fn restart(&self) {
        unsafe { self.manager.startUpdatingLocation() };
    }

    fn poll(
        feed: Rc<LocationFeed>,
        mut position: Signal<Option<Sighting>>,
        mut spread: Signal<Option<Distance>>,
        mut access: Signal<LocationAccess>,
    ) {
        spawn(async move {
            let granted = feed.access();
            let before = *access.peek();
            (granted != before).then(|| {
                access.set(granted);
                feed.restart();
            });
            let previous = *position.peek();
            let latest = feed
                .latest()
                .or_else(|| previous.map(|sighting| sighting.aged(feed.clock.time())));
            let passed = previous.zip(latest).map_or(latest, |(kept, fix)| {
                Some(kept.kept_over(fix, Distance::from(Ground::FixStep)))
            });
            (previous != passed).then(|| position.set(passed));
            let widest = feed.spread();
            let known = *spread.peek();
            (known != widest).then(|| spread.set(widest));
            tokio::time::sleep(Cadence::Location.every()).await;
            Self::poll(feed, position, spread, access);
        });
    }

    fn turn(feed: Rc<LocationFeed>, mut heading: Signal<Option<Bearing>>) {
        spawn(async move {
            let previous = *heading.peek();
            let latest = feed.facing();
            (previous != latest).then(|| heading.set(latest));
            tokio::time::sleep(Cadence::Heading.every()).await;
            Self::turn(feed, heading);
        });
    }

    fn start(clock: WallClock) -> LocationFeed {
        let manager = unsafe { CLLocationManager::new() };
        unsafe {
            manager.setDesiredAccuracy(kCLLocationAccuracyBest);
            manager.setActivityType(CLActivityType::Fitness);
            manager.setPausesLocationUpdatesAutomatically(false);
            manager.setHeadingFilter(Angle::HeadingStep.bearing().degrees());
            manager.requestWhenInUseAuthorization();
            manager.startUpdatingLocation();
        }
        LocationFeed { manager, clock }
    }

    pub fn access(&self) -> LocationAccess {
        match unsafe { self.manager.authorizationStatus() } {
            CLAuthorizationStatus::NotDetermined => LocationAccess::Asking,
            CLAuthorizationStatus::Restricted => LocationAccess::Restricted,
            CLAuthorizationStatus::Denied
                if unsafe { CLLocationManager::locationServicesEnabled_class() } =>
            {
                LocationAccess::Denied
            }
            CLAuthorizationStatus::Denied => LocationAccess::ServicesOff,
            _ if unsafe { self.manager.accuracyAuthorization() }
                == CLAccuracyAuthorization::ReducedAccuracy =>
            {
                LocationAccess::Approximate
            }
            _ => LocationAccess::Allowed,
        }
    }

    pub fn spread(&self) -> Option<Distance> {
        let location = unsafe { self.manager.location() }?;
        Some(unsafe { location.horizontalAccuracy() })
            .filter(|meters| *meters >= 0.0)
            .map(Distance::new)
    }

    pub fn latest(&self) -> Option<Sighting> {
        let location = unsafe { self.manager.location() }?;
        let accuracy = unsafe { location.horizontalAccuracy() };
        let coordinate = unsafe { location.coordinate() };
        let taken = unsafe { location.timestamp() };
        let since: f64 = unsafe { objc2::msg_send![&*taken, timeIntervalSinceNow] };
        let unix: f64 = unsafe { objc2::msg_send![&*taken, timeIntervalSince1970] };
        let at = Coordinate::new(
            Degrees::new(coordinate.latitude),
            Degrees::new(coordinate.longitude),
        );
        let fixed_at = TripTime::at_instant(
            UNIX_EPOCH.checked_add(Duration::try_from_secs_f64(unix).ok()?)?,
            self.clock.time().zone(),
        );
        (0.0..=Distance::from(Ground::FixSpread).meters())
            .contains(&accuracy)
            .then(|| {
                if -since <= Cadence::FixAge.every().as_secs_f64() {
                    Sighting::Fresh(at)
                } else {
                    Sighting::Stale {
                        at,
                        since: fixed_at,
                    }
                }
            })
    }

    pub fn facing(&self) -> Option<Bearing> {
        let heading = unsafe { self.manager.heading() }?;
        let degrees = unsafe { heading.trueHeading() };
        let accuracy = unsafe { heading.headingAccuracy() };
        (degrees >= 0.0 && (0.0..=Angle::HeadingDoubt.bearing().degrees()).contains(&accuracy))
            .then(|| Bearing::new(degrees))
    }
}
