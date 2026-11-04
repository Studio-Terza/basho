use std::str::FromStr;

mod top_reading;

use top_reading::TopReading;

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize)]
pub struct TopSample {
    cpu_percent: TopReading,
    mac_energy_impact: TopReading,
}

impl TopSample {
    const HEADER: &str = "PID";

    fn summed(self, row: TopRow) -> TopSample {
        TopSample {
            cpu_percent: self.cpu_percent + row.cpu,
            mac_energy_impact: self.mac_energy_impact + row.power,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TopRow {
    cpu: TopReading,
    power: TopReading,
}

#[derive(Debug)]
struct ShortRow;

impl FromStr for TopRow {
    type Err = ShortRow;

    fn from_str(row: &str) -> Result<TopRow, ShortRow> {
        let cells: Vec<TopReading> = row
            .split_whitespace()
            .skip(1)
            .filter_map(|cell| cell.parse().ok())
            .collect();
        cells
            .first()
            .copied()
            .zip(cells.get(1).copied())
            .map(|(cpu, power)| TopRow { cpu, power })
            .ok_or(ShortRow)
    }
}

impl FromStr for TopSample {
    type Err = std::convert::Infallible;

    fn from_str(printed: &str) -> Result<TopSample, Self::Err> {
        let rows: Vec<&str> = printed.lines().collect();
        let last = rows
            .iter()
            .rposition(|row| row.trim_start().starts_with(Self::HEADER))
            .unwrap_or(rows.len());
        Ok(rows
            .iter()
            .skip(last + 1)
            .filter_map(|row| row.parse::<TopRow>().ok())
            .fold(TopSample::default(), TopSample::summed))
    }
}
