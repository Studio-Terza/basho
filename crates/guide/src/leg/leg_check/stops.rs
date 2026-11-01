use crate::geo::Coordinate;
use crate::geo::Distance;
use crate::itinerary::Problem;
use crate::leg::leg_check::leg_rule::LegRule;
use crate::leg::{Leg, Mode};
use crate::place::RubyText;
use crate::reading::Reading;

pub struct Stops;

impl Stops {
    const STOP_SLACK: Distance = Distance::new(400.0);
    const STOP_LENGTH_SLACK: Distance = Distance::new(100.0);
    const PAIR_LENGTH: usize = 2;
}

impl LegRule for Stops {
    fn problems(&self, leg: &Leg) -> Vec<Problem> {
        let order = leg
            .along()
            .windows(Self::PAIR_LENGTH)
            .filter(|pair| pair[1].distance_along() < pair[0].distance_along())
            .map(|pair| Problem::StopsOutOfOrder {
                stop: pair[1].distance_along(),
            });
        let range = leg
            .along()
            .iter()
            .filter(|stop| {
                stop.distance_along() < Distance::default()
                    || stop.distance_along() > leg.length() + Self::STOP_LENGTH_SLACK
            })
            .map(|stop| Problem::StopOutside {
                stop: stop.distance_along(),
                length: leg.length(),
            });
        let far = leg.along().iter().filter_map(|stop| {
            let off = leg
                .line()
                .fix(Coordinate::from(stop))
                .map_or(Distance::default(), |fix| Distance::from(fix.off_route()));
            (leg.mode() == Mode::Walk && off > Self::STOP_SLACK).then(|| Problem::StopOffLine {
                name: AsRef::<RubyText>::as_ref(stop).text(Reading::Written),
                off,
            })
        });
        order.chain(range).chain(far).collect()
    }
}
