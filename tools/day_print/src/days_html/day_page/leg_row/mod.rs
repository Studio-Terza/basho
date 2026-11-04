mod board_markup;
mod reason_markup;
mod time_markup;

use guide::card::CardLine;
use guide::reading::Reading;
use guide::words::Label;
use std::fmt;

use guide::leg::{BranchLine, Leg, Mode};

use guide::end::End;
use guide::geo::RouteLine;

use guide::leg_line::LegLine;

use guide::place::NoteKind;

use super::escaped::Escaped;
use super::fact_markup::FactMarkup;
use super::leg_number::LegNumber;
use super::ruby_markup::RubyMarkup;
use board_markup::BoardMarkup;
use reason_markup::ReasonMarkup;
use time_markup::TimeMarkup;

pub struct LegRow<'leg> {
    number: LegNumber,
    leg: &'leg Leg,
}

impl<'leg> LegRow<'leg> {
    #[must_use]
    pub fn new(number: LegNumber, leg: &'leg Leg) -> LegRow<'leg> {
        LegRow { number, leg }
    }

    fn arrival(&self) -> Label {
        match self.leg.mode() {
            Mode::Walk => Label::new("Get to"),
            Mode::Train | Mode::Bus | Mode::Boat | Mode::Car => Label::new("Get off at"),
        }
    }

    fn details(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.leg.no_mapped_paths().try_for_each(|stretch| {
            let line: &RouteLine = stretch.as_ref();
            write!(
                formatter,
                "<br><b>⚠ No mapped path</b> for {}. The line here is on no known road.",
                line.length()
            )
        })?;
        self.leg
            .notes()
            .iter()
            .filter(|note| note.kind() == NoteKind::Warning)
            .try_for_each(|note| write!(formatter, "<br><b>⚠</b> {}", FactMarkup::new(note)))?;
        formatter.write_str("<ul>")?;
        self.leg.along().iter().try_for_each(|stop| {
            write!(
                formatter,
                "<li>{:.1} km · {}</li>",
                stop.distance_along().kilometers(),
                RubyMarkup::new(stop.as_ref())
            )
        })?;
        formatter.write_str("</ul>")?;
        self.leg.branches().iter().try_for_each(|branch| {
            write!(
                formatter,
                "<div class=\"decision\"><b>{}: if</b> {} → {}<br><b>Else</b> {}</div>",
                RubyMarkup::new(branch.line_text(BranchLine::Spot)),
                RubyMarkup::new(branch.line_text(BranchLine::Trigger)),
                RubyMarkup::new(branch.line_text(BranchLine::Backup)),
                RubyMarkup::new(branch.line_text(BranchLine::Main))
            )
        })?;
        self.leg.cards().iter().try_for_each(|card| {
            write!(
                formatter,
                "<div class=\"card\"><p class=\"purpose\">Show this: {}</p><p class=\"japanese\">{}</p><p>{}</p></div>",
                Escaped::new(card.line_text(CardLine::Purpose).text(Reading::Written).as_ref()),
                RubyMarkup::new(card.line_text(CardLine::Japanese)),
                Escaped::new(card.line_text(CardLine::English).text(Reading::Written).as_ref())
            )
        })
    }
}

impl fmt::Display for LegRow<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let leg = self.leg;
        write!(
            formatter,
            "<tr><td class=\"num\">{}</td><td class=\"time\">{}–{}</td><td>",
            self.number,
            TimeMarkup::new(leg.listed(End::Start)),
            TimeMarkup::new(leg.listed(End::Finish)),
        )?;
        leg.conveyance().train().map_or(Ok(()), |train| {
            write!(formatter, "{}", BoardMarkup::new(train))
        })?;
        write!(
            formatter,
            "<b>{} · {} {}</b> <span class=\"stats\">{:.1} km · {}</span><br><b>{} {}.</b> {}<br>{}",
            leg.mode().label(),
            RubyMarkup::new(leg.line_text(LegLine::Way)),
            RubyMarkup::new(leg.line_text(LegLine::Heading)),
            leg.length().kilometers(),
            leg.minutes(),
            self.arrival(),
            RubyMarkup::new(leg.line_text(LegLine::Reach)),
            RubyMarkup::new(leg.line_text(LegLine::Why)),
            RubyMarkup::new(leg.line_text(LegLine::Handrail)),
        )?;
        leg.road().map_or(Ok(()), |road| {
            write!(formatter, " {}", RubyMarkup::new(road.as_ref()))
        })?;
        leg.route_reason().map_or(Ok(()), |reason| {
            write!(formatter, "{}", ReasonMarkup::new(reason))
        })?;
        write!(
            formatter,
            "<br><b>Then:</b> {}<br><b>See:</b> {}",
            RubyMarkup::new(leg.line_text(LegLine::Then)),
            FactMarkup::new(leg.see()),
        )?;
        self.details(formatter)?;
        formatter.write_str("</td></tr>")
    }
}
