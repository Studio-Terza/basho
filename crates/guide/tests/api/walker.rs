use guide::clock::{ClockTime, Minutes, TripTime};
use guide::end::End;
use guide::geo::{Coordinate, Degrees, Distance, Share};
use guide::guidance::{Guidance, LegProgress, Pointer};
use guide::itinerary::Day;
use guide::leg::{Leg, LegIndex, Mode};
use guide::reading::Reading;
use guide::readout::Recovery;

pub struct Walker<'day> {
    day: &'day Day,
    progress: LegProgress,
}

impl<'day> Walker<'day> {
    const STRIDE: Distance = Distance::new(10.0);
    const TOWARD: Degrees = Degrees::new(90.0);

    pub fn fresh(day: &'day Day) -> Walker<'day> {
        Walker::kept(day, LegProgress::default())
    }

    pub fn kept(day: &'day Day, progress: LegProgress) -> Walker<'day> {
        Walker { day, progress }
    }

    pub fn on_step(day: &'day Day, walk: LegIndex, late: Minutes) -> Walker<'day> {
        Walker::kept(day, LegProgress::confirmed(day.number(), walk, late))
    }

    pub fn misleading_sights(day: &'day Day, aside: Distance) -> Vec<String> {
        day.legs()
            .iter()
            .enumerate()
            .filter(|(_, leg)| leg.mode() == Mode::Walk)
            .flat_map(|(position, _)| Self::misleading_on(day, LegIndex::new(position), aside))
            .collect()
    }

    pub fn misleading_on(day: &'day Day, walk: LegIndex, aside: Distance) -> Vec<String> {
        let leg = day.leg(walk).expect("a walk");
        let half = leg.length() * Share::HALF;
        let guidance = Self::beside(leg, half, aside).map(|here| {
            Walker::on_step(day, walk, Minutes::ZERO)
                .fix(Self::planned(leg, half, Minutes::ZERO), here)
        });
        let recovery = guidance.and_then(|guidance| guidance.recovery());
        let route = recovery
            .as_ref()
            .and_then(Recovery::route)
            .map(Pointer::bearing);
        recovery
            .iter()
            .flat_map(|recovery| recovery.toward().to_vec())
            .filter(|sight| {
                route.is_some_and(|route| {
                    sight.pointer().bearing().gap_to(route) > Self::TOWARD.degrees()
                })
            })
            .map(|sight| {
                format!(
                    "day {} leg {walk}: {}",
                    day.number(),
                    sight.name().text(Reading::Written)
                )
            })
            .collect()
    }

    fn planned(leg: &Leg, along: Distance, late: Minutes) -> ClockTime {
        leg.time(End::Start)
            .later_by(leg.planned_after(along) + late)
    }

    pub fn beside(leg: &Leg, along: Distance, aside: Distance) -> Option<Coordinate> {
        let back = leg
            .line()
            .point_along((along - Self::STRIDE).max(Distance::default()))?;
        let next = leg.line().point_along(along + Self::STRIDE)?;
        let east = next.lon().east_of(back).meters();
        let north = next.lat().north_of(back).meters();
        let span = east.hypot(north).max(f64::EPSILON);
        Some(
            leg.line()
                .point_along(along)?
                .offset_by(aside * (-north / span), aside * (east / span)),
        )
    }

    pub fn fix(&self, time: ClockTime, here: Coordinate) -> Guidance<'day> {
        let now = TripTime::on_date_at(self.day.zone(), self.day.date(), time);
        Guidance::new(self.day, self.progress, now, Some(here))
    }
}
