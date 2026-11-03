use crate::itinerary::Problem;
use crate::leg::leg_check::leg_rule::LegRule;
use crate::leg::{BranchLine, Leg};
use crate::leg_line::LegLine;
use crate::ruby_text::RubyText;
use crate::words::Label;

pub struct Lengths;

impl Lengths {
    const WAY_LENGTH: usize = 40;
    const HEADING_LENGTH: usize = 30;
    const HANDRAIL_LENGTH: usize = 100;
    const THEN_LENGTH: usize = 110;
    const SEE_LENGTH: usize = 110;
    const REACH_LENGTH: usize = 28;
    const WHY_LENGTH: usize = 160;
    const ROAD_LENGTH: usize = 200;
    const STOP_NAME_LENGTH: usize = 60;
    const STOP_DETAIL_LENGTH: usize = 100;
    const SPOT_LENGTH: usize = 30;

    fn beyond_length(label: &'static str, text: &RubyText, length_limit: usize) -> Option<Problem> {
        let length = text.char_length();
        (length > length_limit).then_some(Problem::TooLong {
            label: Label::new(label),
            length,
            length_limit,
        })
    }
}

impl LegRule for Lengths {
    fn problems(&self, leg: &Leg) -> Vec<Problem> {
        let stops = leg.along().iter().flat_map(|stop| {
            [
                Self::beyond_length("a stop name", stop.as_ref(), Self::STOP_NAME_LENGTH),
                stop.detail().and_then(|detail| {
                    Self::beyond_length("a stop detail", detail.text(), Self::STOP_DETAIL_LENGTH)
                }),
            ]
        });
        let spots = leg.branches().iter().map(|branch| {
            Self::beyond_length(
                "a branch's where",
                branch.line_text(BranchLine::Spot),
                Self::SPOT_LENGTH,
            )
        });
        [
            Self::beyond_length("way", leg.line_text(LegLine::Way), Self::WAY_LENGTH),
            Self::beyond_length(
                "heading",
                leg.line_text(LegLine::Heading),
                Self::HEADING_LENGTH,
            ),
            Self::beyond_length(
                "handrail",
                leg.line_text(LegLine::Handrail),
                Self::HANDRAIL_LENGTH,
            ),
            Self::beyond_length("then", leg.line_text(LegLine::Then), Self::THEN_LENGTH),
            Self::beyond_length("see", leg.see().text(), Self::SEE_LENGTH),
            Self::beyond_length("reach", leg.line_text(LegLine::Reach), Self::REACH_LENGTH),
            Self::beyond_length("why", leg.line_text(LegLine::Why), Self::WHY_LENGTH),
            leg.road()
                .and_then(|road| Self::beyond_length("road", road.as_ref(), Self::ROAD_LENGTH)),
        ]
        .into_iter()
        .chain(stops)
        .chain(spots)
        .flatten()
        .collect()
    }
}
