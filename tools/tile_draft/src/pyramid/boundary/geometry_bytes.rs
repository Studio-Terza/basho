use tile_shapes::GridPoint;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GeometryBytes {
    numbers: Vec<u64>,
    pen: GridPoint,
}

impl GeometryBytes {
    const MOVE: u64 = 1;
    const DRAW: u64 = 2;
    const CLOSE: u64 = 7;

    #[must_use]
    pub fn numbers(&self) -> &[u64] {
        &self.numbers
    }

    pub fn path(&mut self, points: &[GridPoint]) {
        let (first, rest) = points.split_at(1);
        self.run(Self::MOVE, first);
        self.run(Self::DRAW, rest);
    }

    pub fn close_for(&mut self, closed: bool) {
        self.numbers
            .extend(closed.then_some(Self::command(Self::CLOSE, 1)));
    }

    #[expect(
        clippy::needless_for_each,
        reason = "expression-flow drives iterators with combinators, never a for loop"
    )]
    fn run(&mut self, command: u64, points: &[GridPoint]) {
        let count = u64::try_from(points.len()).unwrap_or(u64::MAX);
        self.numbers.push(Self::command(command, count));
        points.iter().for_each(|point| {
            self.numbers.extend(point.step_from(self.pen));
            self.pen = *point;
        });
    }

    const fn command(command: u64, count: u64) -> u64 {
        command | (count << 3)
    }
}
