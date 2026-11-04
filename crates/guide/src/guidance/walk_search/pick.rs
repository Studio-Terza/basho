use crate::clock::{Minutes, TripTime};
use crate::geo::{Coordinate, Distance, Share};
use crate::guidance::leg_progress::LegProgress;
use crate::guidance::nearness::Nearness;
use crate::guidance::snap::Snap;
use crate::guidance::walk_search::stride::Stride;
use crate::itinerary::Day;

#[derive(Clone, Copy, Debug)]
pub struct Pick<'day> {
    day: &'day Day,
    here: Coordinate,
    now: TripTime,
    kept: LegProgress,
    blur: Nearness,
    pass: Minutes,
}

impl<'day> Pick<'day> {
    const CATCH_UP: Share = Share::new(1.0 / 3.0);
    const PASS_SPREAD: Minutes = Minutes::new(1);
    const STEADY: Minutes = Minutes::new(2);
    const SAME_PASS: Minutes = Minutes::new(10);
    const CLOCK_STEP: Minutes = Minutes::new(1);

    #[must_use]
    pub fn new(day: &'day Day, here: Coordinate, now: TripTime, kept: LegProgress) -> Pick<'day> {
        Pick {
            day,
            here,
            now,
            kept,
            blur: Nearness::GpsError,
            pass: Self::SAME_PASS,
        }
    }

    #[must_use]
    pub fn unseen(day: &'day Day, here: Coordinate, now: TripTime) -> Pick<'day> {
        Pick {
            blur: Nearness::SameRoad,
            pass: Self::PASS_SPREAD,
            ..Pick::new(day, here, now, LegProgress::dawn(day))
        }
    }

    #[must_use]
    pub fn best(self, pool: &[Snap]) -> Option<Snap> {
        let near = Self::closest(pool, self.blur);
        let drift = |snap: &Snap| self.drift(snap.delay(self.day, self.now));
        let least = near.iter().map(drift).min()?;
        near.into_iter()
            .filter(|snap| drift(snap) <= least + Self::PASS_SPREAD)
            .min_by(|left, right| {
                self.cost(*left)
                    .meters()
                    .total_cmp(&self.cost(*right).meters())
                    .then_with(|| self.slip(*left).cmp(&self.slip(*right)))
                    .then_with(|| left.index().cmp(&right.index()))
                    .then_with(|| {
                        left.fix()
                            .along()
                            .meters()
                            .total_cmp(&right.fix().along().meters())
                    })
            })
    }

    #[must_use]
    pub fn slip(self, snap: Snap) -> Minutes {
        (snap.delay(self.day, self.now) - self.kept.offset()).abs()
    }

    fn cost(self, snap: Snap) -> Distance {
        let error = Nearness::GpsError.distance();
        let elapsed = self.kept.minutes_before(self.day, self.now);
        let unsure = Stride::BRISK.distance_in((elapsed - Self::CLOCK_STEP).max(Minutes::ZERO));
        let weight = self
            .kept
            .moved_to(self.here)
            .map_or(0.0, |_| error / (error + unsure));
        let slipped = error * (self.slip(snap) - self.pass).max(Minutes::ZERO).amount();
        Distance::from(snap.fix().off_route())
            + slipped * (1.0 - weight)
            + Stride::between(self.day, self.kept, snap, self.now)
                .map_or(Nearness::ClearlyElsewhere.distance(), |stride| {
                    self.gap(stride) * weight
                })
    }

    fn closest(pool: &[Snap], alike: Nearness) -> Vec<Snap> {
        pool.iter()
            .map(|snap| Distance::from(snap.fix().off_route()))
            .min_by(|left, right| left.meters().total_cmp(&right.meters()))
            .map_or_else(Vec::new, |best| {
                pool.iter()
                    .copied()
                    .filter(|snap| snap.fix().off_route() <= best + alike.distance())
                    .collect()
            })
    }

    fn drift(self, delay: Minutes) -> Minutes {
        let elapsed = self.kept.minutes_before(self.day, self.now);
        let earliest = self.kept.offset()
            - Minutes::rounded(elapsed.amount() * Self::CATCH_UP.fraction()).unwrap_or_default();
        let latest = self.kept.offset() + elapsed;
        let early = Minutes::rounded((earliest - delay).amount() / Self::CATCH_UP.fraction())
            .unwrap_or_default();
        Some(elapsed)
            .filter(|elapsed| *elapsed > Self::STEADY)
            .map_or(Minutes::ZERO, |_| {
                early.max(delay - latest).max(Minutes::ZERO)
            })
    }

    fn gap(self, stride: Stride) -> Distance {
        let moved = self.kept.moved_to(self.here).unwrap_or_default();
        let stepped = stride.stepped();
        Some(stepped)
            .filter(|stepped| *stepped < Distance::default())
            .map_or_else(
                || (stepped - moved).abs(),
                |stepped| moved - stepped * Self::CATCH_UP.fraction().recip(),
            )
    }
}
