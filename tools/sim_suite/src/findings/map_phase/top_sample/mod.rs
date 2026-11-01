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

    fn summed(self, row: &str) -> TopSample {
        let cells: Vec<TopReading> = row
            .split_whitespace()
            .skip(1)
            .filter_map(|cell| cell.parse().ok())
            .collect();
        match cells.as_slice() {
            [cpu, power, ..] => TopSample {
                cpu_percent: self.cpu_percent + *cpu,
                mac_energy_impact: self.mac_energy_impact + *power,
            },
            _ => self,
        }
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
            .fold(TopSample::default(), |sample, row| sample.summed(row)))
    }
}
