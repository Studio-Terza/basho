use guide::BufferState;
use guide::clock::{ClockTime, Minutes, TripTime};
use guide::end::End;
use guide::geo::{Coordinate, Share};
use guide::guidance::timeline::{Buffer, Halt};
use guide::guidance::{Guidance, LegProgress};
use guide::itinerary::{Day, Itinerary};
use guide::leg::{Leg, LegIndex};

use super::Wanted;

#[derive(Clone, Copy)]
pub struct WalkScene<'trip> {
    day: &'trip Day,
    index: LegIndex,
    leg: &'trip Leg,
    delay: Minutes,
}

impl<'trip> WalkScene<'trip> {
    const TIGHT_SPARE: Minutes = Minutes::new(5);
    const SHORT_BY: Minutes = Minutes::new(1);
    pub const STOPPED_FOR: Minutes = Minutes::new(25);

    pub fn all(itinerary: &'trip Itinerary) -> Vec<WalkScene<'trip>> {
        itinerary
            .days()
            .iter()
            .flat_map(|day| {
                day.legs()
                    .iter()
                    .enumerate()
                    .filter(|(_, leg)| {
                        leg.mode() == guide::leg::Mode::Walk && leg.line().is_sketch().is_none()
                    })
                    .map(move |(position, leg)| WalkScene {
                        day,
                        index: LegIndex::new(position),
                        leg,
                        delay: Minutes::ZERO,
                    })
            })
            .collect()
    }

    pub fn first(
        scenes: &[WalkScene<'trip>],
        state: BufferState,
        wanted: Wanted,
    ) -> Option<WalkScene<'trip>> {
        scenes.iter().find_map(|scene| {
            scene
                .delay_for(state, wanted)
                .map(|delay| WalkScene { delay, ..*scene })
        })
    }

    pub fn time(&self) -> ClockTime {
        self.leg
            .time(End::Start)
            .later_by(
                self.leg
                    .planned_after(self.leg.line().length() * Share::HALF),
            )
            .later_by(self.delay)
    }

    pub fn day(&self) -> &'trip Day {
        self.day
    }

    pub fn leg(&self) -> &'trip Leg {
        self.leg
    }

    pub fn progress(&self) -> LegProgress {
        LegProgress::confirmed(self.day.number(), self.index, Minutes::ZERO)
    }

    pub fn delay_for(&self, state: BufferState, wanted: Wanted) -> Option<Minutes> {
        let spare = self.buffer_at(Minutes::ZERO)?.spare();
        let delay = match state {
            BufferState::Comfortable | BufferState::Waiting => Minutes::ZERO,
            BufferState::Tight => spare - Self::TIGHT_SPARE,
            BufferState::Short => spare + Self::SHORT_BY,
        };
        self.buffer_at(delay)
            .filter(|buffer| delay >= Minutes::ZERO && buffer.state() == state)
            .and_then(|buffer| wanted.kept(buffer))
            .map(|_| delay)
    }

    pub fn stopped(&self) -> Option<Halt> {
        let at = self.leg.point(End::Finish);
        let since =
            TripTime::on_date_at(self.day.zone(), self.day.date(), self.leg.time(End::Finish));
        let now = since.later_by(Self::STOPPED_FOR);
        let halt = Halt::tracked(None, at, since);
        Guidance::new(self.day, self.progress(), now, Some(at))
            .paused(Some(halt))
            .stop()
            .filter(|stop| stop.planned().booked() > Self::STOPPED_FOR)
            .map(|_| halt)
    }

    fn buffer_at(&self, delay: Minutes) -> Option<Buffer<'trip>> {
        let here: Option<Coordinate> = self
            .leg
            .line()
            .point_along(self.leg.line().length() * Share::HALF);
        let now = TripTime::on_date_at(
            self.day.zone(),
            self.day.date(),
            WalkScene { delay, ..*self }.time(),
        );
        Guidance::new(self.day, self.progress(), now, here).buffer()
    }
}
