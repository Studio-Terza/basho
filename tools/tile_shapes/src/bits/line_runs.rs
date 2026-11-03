use super::WorldPoint;
use tiles::GridAxis;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RunEnd {
    Goes,
    Stops,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LineRuns {
    finished: Vec<Vec<WorldPoint>>,
    open: Vec<WorldPoint>,
}

impl LineRuns {
    const LEAST: f64 = -64.0;
    const MOST: f64 = WorldPoint::EXTENT + 64.0;

    #[must_use]
    pub fn inside(line: &[WorldPoint]) -> Vec<Vec<WorldPoint>> {
        let runs = line
            .iter()
            .zip(line.iter().skip(1))
            .fold(LineRuns::default(), |runs, (start, finish)| {
                runs.with_step(*start, *finish)
            })
            .stopped();
        runs.finished
    }

    fn with_step(self, start: WorldPoint, finish: WorldPoint) -> LineRuns {
        match Self::span(start, finish) {
            Some([from, to]) if from <= to => {
                let end = if to < 1.0 {
                    RunEnd::Stops
                } else {
                    RunEnd::Goes
                };
                let joined = self.joined(start.toward(finish, from), start.toward(finish, to));
                if end == RunEnd::Stops {
                    joined.stopped()
                } else {
                    joined
                }
            }
            _ => self.stopped(),
        }
    }

    fn joined(mut self, from: WorldPoint, to: WorldPoint) -> LineRuns {
        self.open.extend(self.open.is_empty().then_some(from));
        self.open.push(to);
        self
    }

    fn stopped(mut self) -> LineRuns {
        let run = std::mem::take(&mut self.open);
        self.finished.extend((!run.is_empty()).then_some(run));
        self
    }

    fn span(start: WorldPoint, finish: WorldPoint) -> Option<[f64; 2]> {
        let along = finish - start;
        [GridAxis::Across, GridAxis::Down]
            .into_iter()
            .flat_map(|axis| {
                [
                    [-along.along(axis), start.along(axis) - Self::LEAST],
                    [along.along(axis), Self::MOST - start.along(axis)],
                ]
            })
            .try_fold([0.0, 1.0], |[from, to], [rate, room]| {
                if rate == 0.0 {
                    (room >= 0.0).then_some([from, to])
                } else if rate < 0.0 {
                    Some([f64::max(from, room / rate), to])
                } else {
                    Some([from, f64::min(to, room / rate)])
                }
            })
    }
}
