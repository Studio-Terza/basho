use crate::itinerary::Problem;
use crate::leg::leg_check::leg_rule::LegRule;
use crate::leg::{BranchLine, Leg};
use crate::leg_line::LegLine;
use crate::ruby_text::{LetterCount, RubyText};
use crate::words::Label;

pub struct Lengths;

impl Lengths {
    const WAY_LENGTH: LetterCount = LetterCount::new(40);
    const HEADING_LENGTH: LetterCount = LetterCount::new(30);
    const HANDRAIL_LENGTH: LetterCount = LetterCount::new(100);
    const THEN_LENGTH: LetterCount = LetterCount::new(110);
    const SEE_LENGTH: LetterCount = LetterCount::new(110);
    const REACH_LENGTH: LetterCount = LetterCount::new(28);
    const WHY_LENGTH: LetterCount = LetterCount::new(160);
    const ROAD_LENGTH: LetterCount = LetterCount::new(200);
    const STOP_NAME_LENGTH: LetterCount = LetterCount::new(60);
    const STOP_DETAIL_LENGTH: LetterCount = LetterCount::new(100);
    const SPOT_LENGTH: LetterCount = LetterCount::new(30);

    fn beyond_length(label: Label, text: &RubyText, length_limit: LetterCount) -> Option<Problem> {
        let length = LetterCount::from(text);
        (length > length_limit).then_some(Problem::TooLong {
            label,
            length,
            length_limit,
        })
    }
}

impl LegRule for Lengths {
    fn problems(&self, leg: &Leg) -> Vec<Problem> {
        let stops = leg.along().iter().flat_map(|stop| {
            [
                Self::beyond_length(
                    Label::new("a stop name"),
                    stop.as_ref(),
                    Self::STOP_NAME_LENGTH,
                ),
                stop.detail().and_then(|detail| {
                    Self::beyond_length(
                        Label::new("a stop detail"),
                        detail.text(),
                        Self::STOP_DETAIL_LENGTH,
                    )
                }),
            ]
        });
        let spots = leg.branches().iter().map(|branch| {
            Self::beyond_length(
                Label::new("a branch's where"),
                branch.line_text(BranchLine::Spot),
                Self::SPOT_LENGTH,
            )
        });
        [
            Self::beyond_length(
                Label::new("way"),
                leg.line_text(LegLine::Way),
                Self::WAY_LENGTH,
            ),
            Self::beyond_length(
                Label::new("heading"),
                leg.line_text(LegLine::Heading),
                Self::HEADING_LENGTH,
            ),
            Self::beyond_length(
                Label::new("handrail"),
                leg.line_text(LegLine::Handrail),
                Self::HANDRAIL_LENGTH,
            ),
            Self::beyond_length(
                Label::new("then"),
                leg.line_text(LegLine::Then),
                Self::THEN_LENGTH,
            ),
            Self::beyond_length(Label::new("see"), leg.see().text(), Self::SEE_LENGTH),
            Self::beyond_length(
                Label::new("reach"),
                leg.line_text(LegLine::Reach),
                Self::REACH_LENGTH,
            ),
            Self::beyond_length(
                Label::new("why"),
                leg.line_text(LegLine::Why),
                Self::WHY_LENGTH,
            ),
            leg.road().and_then(|road| {
                Self::beyond_length(Label::new("road"), road.as_ref(), Self::ROAD_LENGTH)
            }),
        ]
        .into_iter()
        .chain(stops)
        .chain(spots)
        .flatten()
        .collect()
    }
}
