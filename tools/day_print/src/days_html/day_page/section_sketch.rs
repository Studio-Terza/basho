use std::fmt;

use guide::map_frame::MapFrame;
use guide::plane::Axis;
use guide::plane::Pixels;

use guide::geo::{Coordinate, RouteLine};

use guide::end::End;

use guide::leg::{Leg, Mode};

use super::leg_number::LegNumber;

pub struct SectionSketch<'day> {
    first: LegNumber,
    legs: &'day [Leg],
}

impl<'day> SectionSketch<'day> {
    const MAP_WIDTH: Pixels = Pixels::new(640.0);
    const LABEL_OFFSET: Pixels = Pixels::new(6.0);

    #[must_use]
    pub fn new(first: LegNumber, legs: &'day [Leg]) -> SectionSketch<'day> {
        SectionSketch { first, legs }
    }

    #[must_use]
    pub fn walks(&self) -> bool {
        self.legs
            .first()
            .is_some_and(|leg| leg.mode() == Mode::Walk)
    }

    fn drawn(&self, frame: &MapFrame, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.legs
            .iter()
            .zip(self.first.onward())
            .try_for_each(|(leg, number)| {
                let end = frame.project(leg.point(End::Finish));
                write!(
                    formatter,
                    "<path d=\"{}\"/><circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4\"/><text x=\"{:.1}\" y=\"{:.1}\">{number}</text>",
                    frame.svg_path(leg.line().points()),
                    end[Axis::Across].pixels(),
                    end[Axis::Down].pixels(),
                    (end[Axis::Across] + Self::LABEL_OFFSET).pixels(),
                    (end[Axis::Down] - Self::LABEL_OFFSET).pixels(),
                )?;
                leg.no_mapped_paths().try_for_each(|stretch| {
                    let line: &RouteLine = stretch.as_ref();
                    write!(
                        formatter,
                        "<path class=\"no-mapped-path\" d=\"{}\"/>",
                        frame.svg_path(line.points())
                    )
                })
            })
    }
}

impl fmt::Display for SectionSketch<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let framed: Vec<Coordinate> = self
            .legs
            .iter()
            .flat_map(|leg| leg.line().points().iter().copied())
            .collect();
        MapFrame::around(&framed, Self::MAP_WIDTH).map_or(Ok(()), |frame| {
            write!(
                formatter,
                "<svg class=\"sketch\" viewBox=\"0 0 {:.0} {:.0}\">",
                frame.extent(Axis::Across).pixels(),
                frame.extent(Axis::Down).pixels()
            )?;
            self.drawn(&frame, formatter)?;
            formatter.write_str("</svg>")
        })
    }
}
