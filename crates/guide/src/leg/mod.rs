pub mod along;
mod branch;
pub mod card;
mod card_fill;
pub mod conveyance;
mod cue;
mod cutoff;
mod gait;
mod leg_check;
mod leg_index;
mod margin_rule;
mod pay;
mod road;
mod stop_kind;
mod tour;
mod transfer;
use crate::words::Slug;
use serde::Deserialize;

use crate::clock::{ClockTime, Minutes};
use crate::content::{Content, ContentList};
use crate::end::End;
use crate::geo::{Coordinate, Distance, RouteLine};
use crate::itinerary::{DayNumber, Finding};
use crate::leg_line::LegLine;
use crate::place::Fact;
use crate::ruby_text::RubyText;
use crate::timeline::Margin;
use crate::times::ListedTime;
use crate::way::{LegPath, NoMappedPath, WalkSegment};
use along::Along;
use conveyance::{Conveyance, RouteReason, TrainKind};
use gait::Gait;
use leg_check::LegCheck;
use margin_rule::MarginRule;
use pay::Pay;
use road::Road;
use tour::Tour;
use transfer::Transfer;

use crate::clock::Pause;
pub use branch::{Branch, BranchLine};
pub use card::Card;
use card::CardLine;
pub use card_fill::CardFill;
pub use conveyance::Mode;
pub use cutoff::Cutoff;
pub use leg_index::LegIndex;
pub use stop_kind::StopKind;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Leg {
    #[serde(rename = "mode")]
    conveyance: Conveyance,
    way: Content,
    heading: Content,
    handrail: Content,
    #[serde(rename = "km", deserialize_with = "Distance::deserialize_kilometers")]
    length: Distance,
    minutes: Minutes,
    start: ListedTime,
    end: ListedTime,
    dwell: Option<Minutes>,
    then: Content,
    reach: Content,
    why: Content,
    road: Option<Road>,
    see: Fact,
    at: Coordinate,
    from: Option<Coordinate>,
    to: Option<RubyText>,
    place: Option<Slug>,
    pay: Option<Pay>,
    margin: Option<MarginRule>,
    #[serde(default)]
    gait: Gait,
    transfer: Option<Transfer>,
    tour: Option<Tour>,
    cutoff: Option<Cutoff>,
    #[serde(default)]
    notes: Vec<Fact>,
    #[serde(default)]
    along: Vec<Along>,
    #[serde(default)]
    branches: Vec<Branch>,
    #[serde(default)]
    cards: Vec<Card>,
    #[serde(skip)]
    line: RouteLine,
    #[serde(skip)]
    segments: Vec<WalkSegment>,
}

impl Leg {
    #[must_use]
    pub fn attached(self, path: &LegPath) -> Leg {
        Leg {
            line: path.as_ref().clone(),
            segments: path.segments().to_vec(),
            ..self
        }
    }

    #[must_use]
    pub fn mode(&self) -> Mode {
        Mode::from(&self.conveyance)
    }

    #[must_use]
    pub fn conveyance(&self) -> &Conveyance {
        &self.conveyance
    }

    #[must_use]
    pub fn route_reason(&self) -> Option<&RouteReason> {
        self.conveyance.walk().map(AsRef::as_ref)
    }

    #[must_use]
    pub fn line_text(&self, line: LegLine) -> &Content {
        match line {
            LegLine::Way => &self.way,
            LegLine::Heading => &self.heading,
            LegLine::Handrail => &self.handrail,
            LegLine::Then => &self.then,
            LegLine::Reach => &self.reach,
            LegLine::Why => &self.why,
        }
    }

    #[must_use]
    pub fn length(&self) -> Distance {
        self.length
    }

    #[must_use]
    pub fn minutes(&self) -> Minutes {
        self.minutes
    }

    #[must_use]
    pub fn time(&self, end: End) -> ClockTime {
        ClockTime::from(self.listed(end))
    }

    #[must_use]
    pub fn listed(&self, end: End) -> ListedTime {
        match end {
            End::Start => self.start,
            End::Finish => self.end,
        }
    }

    #[must_use]
    pub fn paused(&self, pause: Pause) -> Minutes {
        match pause {
            Pause::AtEnd => self.dwell.unwrap_or_default(),
            Pause::Along => self.along.iter().map(Along::dwell_minutes).sum(),
        }
    }

    #[must_use]
    pub fn planned_after(&self, along: Distance) -> Minutes {
        let share = if self.length > Distance::default() {
            (along / self.length).clamp(0.0, 1.0)
        } else {
            1.0
        };
        let stops: Minutes = self
            .along
            .iter()
            .filter(|stop| stop.distance_along() < along)
            .map(Along::dwell_minutes)
            .sum();
        Minutes::rounded(self.minutes.amount() * share).unwrap_or_default() + stops
    }

    #[must_use]
    pub fn along_after(&self, elapsed: Minutes) -> Distance {
        let pace = self.minutes.amount().max(1.0) / self.length.meters().max(1.0);
        let walked = |from: Distance, minutes: f64| {
            let reached = from + Distance::new(minutes.max(0.0) / pace);
            if reached > self.length {
                self.length
            } else {
                reached
            }
        };
        let arrives = |stop: &Along| {
            stop.distance_along().meters() * pace
                + self
                    .along
                    .iter()
                    .filter(|earlier| earlier.distance_along() < stop.distance_along())
                    .map(|earlier| earlier.dwell_minutes().amount())
                    .sum::<f64>()
        };
        let last = self
            .along
            .iter()
            .rev()
            .find(|stop| arrives(stop) <= elapsed.amount());
        last.map_or_else(
            || walked(Distance::default(), elapsed.amount()),
            |stop| {
                let left = elapsed.amount() - arrives(stop) - stop.dwell_minutes().amount();
                if left < 0.0 {
                    stop.distance_along()
                } else {
                    walked(stop.distance_along(), left)
                }
            },
        )
    }

    #[must_use]
    pub fn boarding_margin(&self) -> Option<Margin> {
        let pay = self.pay.unwrap_or(Pay::Buy);
        let shinkansen = self
            .conveyance
            .train()
            .map(AsRef::<TrainKind>::as_ref)
            .filter(|kind| **kind == TrainKind::Shinkansen);
        let usual = self
            .transfer
            .map_or(Margin::from(pay), |transfer| match (pay, shinkansen) {
                (Pay::StoredValue | Pay::Held, None) => Margin::from(transfer),
                (Pay::StoredValue | Pay::Held | Pay::Buy | Pay::Pickup | Pay::OnBoard, _) => {
                    Margin::from(pay).stricter(Margin::from(transfer))
                }
            });
        (self.mode() != Mode::Walk).then(|| self.margin.map_or(usual, Margin::from))
    }

    #[must_use]
    pub fn pay(&self) -> Option<Pay> {
        self.pay
    }

    #[must_use]
    pub fn transfer(&self) -> Option<Transfer> {
        self.transfer
    }

    #[must_use]
    pub fn margin_rule(&self) -> Option<MarginRule> {
        self.margin
    }

    #[must_use]
    pub fn gait(&self) -> Gait {
        self.gait
    }

    #[must_use]
    pub fn tour(&self) -> Option<Tour> {
        self.tour
    }

    #[must_use]
    pub fn cutoff(&self) -> Option<&Cutoff> {
        self.cutoff.as_ref()
    }

    #[must_use]
    pub fn see(&self) -> &Fact {
        &self.see
    }

    #[must_use]
    pub fn road(&self) -> Option<&Road> {
        self.road.as_ref()
    }

    #[must_use]
    pub fn place(&self) -> Option<&Slug> {
        self.place.as_ref()
    }

    #[must_use]
    pub fn notes(&self) -> &[Fact] {
        &self.notes
    }

    #[must_use]
    pub fn along(&self) -> &[Along] {
        &self.along
    }

    #[must_use]
    pub fn branches(&self) -> &[Branch] {
        &self.branches
    }

    #[must_use]
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    #[must_use]
    pub fn line(&self) -> &RouteLine {
        &self.line
    }

    #[must_use]
    pub fn segments(&self) -> &[WalkSegment] {
        &self.segments
    }

    pub fn no_mapped_paths(&self) -> impl Iterator<Item = &NoMappedPath> {
        self.segments.iter().filter_map(WalkSegment::no_mapped_path)
    }

    #[must_use]
    pub fn boarding(&self) -> Option<Coordinate> {
        self.from
    }

    #[must_use]
    pub fn alighting(&self) -> Option<&RubyText> {
        self.to.as_ref()
    }

    #[must_use]
    pub fn point(&self, end: End) -> Coordinate {
        match end {
            End::Start => self
                .line
                .points()
                .first()
                .copied()
                .or(self.from)
                .unwrap_or(self.at),
            End::Finish => self.line.points().last().copied().unwrap_or(self.at),
        }
    }

    #[must_use]
    pub fn findings(&self, day: DayNumber, index: LegIndex) -> Vec<Finding> {
        LegCheck::new(self, day, index).findings()
    }
}

impl<'leg> From<&'leg Leg> for ContentList<'leg> {
    fn from(leg: &'leg Leg) -> ContentList<'leg> {
        let branches = leg.branches.iter().flat_map(|branch| {
            [
                BranchLine::Spot,
                BranchLine::Trigger,
                BranchLine::Main,
                BranchLine::Backup,
            ]
            .map(|line| branch.line_text(line))
        });
        let cards = leg.cards.iter().flat_map(|card| {
            [CardLine::Purpose, CardLine::Japanese, CardLine::English]
                .map(|line| card.line_text(line))
        });
        let stops = leg.along.iter().flat_map(|stop| {
            std::iter::once(AsRef::<Content>::as_ref(stop))
                .chain(stop.detail().into_iter().flat_map(ContentList::from))
        });
        let facts = leg
            .notes
            .iter()
            .chain([&leg.see])
            .flat_map(ContentList::from);
        [
            &leg.way,
            &leg.heading,
            &leg.handrail,
            &leg.then,
            &leg.reach,
            &leg.why,
        ]
        .into_iter()
        .chain(leg.road.as_ref().map(AsRef::as_ref))
        .chain(
            leg.route_reason()
                .and_then(RouteReason::referent)
                .map(AsRef::<Content>::as_ref),
        )
        .chain(branches)
        .chain(cards)
        .chain(stops)
        .chain(facts)
        .collect()
    }
}
