use guide::TripSource;
use std::sync::{Arc, OnceLock};

use guide::Trip;
use guide::TripError;
use guide::clock::TripZone;
use guide::itinerary::Finding;
use guide::itinerary::{Day, Itinerary};
use toml::{Table, Value};

#[derive(Clone, Copy, Debug)]
pub struct DaySource<'source> {
    day: &'source str,
    routes: &'source str,
}

impl<'source> DaySource<'source> {
    #[must_use]
    pub const fn new(day: &'source str, routes: &'source str) -> DaySource<'source> {
        DaySource { day, routes }
    }

    fn numbered(json: &str, number: &Value) -> Value {
        let mut table: Table = serde_json::from_str(json).expect("a fixture section");
        table.insert(Fixture::DAY_KEY.to_owned(), number.clone());
        Value::Table(table)
    }
}

pub struct BundledDays(Arc<Itinerary>);

impl BundledDays {
    #[must_use]
    pub fn day(&self, number: usize) -> &Day {
        self.0
            .days()
            .iter()
            .find(|day| usize::from(day.number().number()) == number)
            .expect("the day was asked for")
    }
}

pub struct Fixture;

impl Fixture {
    pub const BUNDLED: &str = include_str!("../../../../data/trips/sample.toml");
    const CATCHES_ONWARD: &str = "\n[kana]\n";
    const DAYS_OPEN: &str = "\n[[days]]\n";
    const ARCS_OPEN: &str = "\n[[arcs]]\n";
    const PLACES_OPEN: &str = "\n[[places]]\n";
    const ROUTES_OPEN: &str = "\n[[routes]]\n";
    const LANDMARKS_OPEN: &str = "\n[[landmarks.";
    const ARC_DAYS: &str = "days = [";
    const HEAD: &str = "name = \"\"\nutc_offset = \"+09:00\"\nlang = \"ja\"\n";
    const NUMBER_KEY: &str = "number";
    const DAY_KEY: &str = "day";

    #[must_use]
    pub fn bundled_routes() -> &'static str {
        Self::BUNDLED
            .split(Self::CATCHES_ONWARD)
            .next()
            .unwrap_or_default()
    }

    pub fn bundled() -> Result<Arc<Itinerary>, TripError> {
        static PARSED: OnceLock<Arc<Itinerary>> = OnceLock::new();
        PARSED.get().map(Arc::clone).map_or_else(
            || {
                Self::bundled_routes()
                    .parse::<Trip>()
                    .map(|trip| Arc::clone(PARSED.get_or_init(|| Arc::clone(trip.itinerary()))))
            },
            Ok,
        )
    }

    pub fn bundled_days(numbers: &[usize]) -> BundledDays {
        let first = numbers.iter().copied().min().unwrap_or_default();
        let last = numbers.iter().copied().max().unwrap_or_default();
        Self::bundled_days_text(&(first..=last).collect::<Vec<usize>>())
            .parse::<Trip>()
            .map(|trip| BundledDays(Arc::clone(trip.itinerary())))
            .expect("the bundled days load alone")
    }

    fn bundled_days_text(numbers: &[usize]) -> String {
        let (head, rest) = Self::bundled_routes()
            .split_once(Self::DAYS_OPEN)
            .expect("the bundled trip has days");
        let (days, rest) = rest
            .split_once(Self::LANDMARKS_OPEN)
            .expect("the bundled trip has landmarks");
        let (landmarks, routes) = rest
            .split_once(Self::ROUTES_OPEN)
            .expect("the bundled trip has routes");
        let (arcs, places) = head
            .split_once(Self::PLACES_OPEN)
            .expect("the bundled trip has places");
        let mut arcs = arcs.split(Self::ARCS_OPEN);
        let trip = arcs.next().unwrap_or_default();
        let arcs: Vec<String> = arcs.filter_map(|arc| Self::kept(arc, numbers)).collect();
        let blocks = |text: &str, open: &str, key: &str| -> Vec<String> {
            numbers
                .iter()
                .map(|number| {
                    let first = format!("{key} = {number}\n");
                    text.split(open)
                        .find(|block| block.starts_with(&first))
                        .expect("the bundled trip has the day")
                        .to_owned()
                })
                .collect()
        };
        let joined = |open: &str, blocks: Vec<String>| {
            blocks
                .iter()
                .flat_map(|block| [open, block.as_str()])
                .collect::<String>()
        };
        [
            trip.to_owned(),
            joined(Self::ARCS_OPEN, arcs),
            Self::PLACES_OPEN.to_owned(),
            places.to_owned(),
            joined(Self::DAYS_OPEN, blocks(days, Self::DAYS_OPEN, "number")),
            Self::LANDMARKS_OPEN.to_owned(),
            landmarks.to_owned(),
            joined(Self::ROUTES_OPEN, blocks(routes, Self::ROUTES_OPEN, "day")),
        ]
        .concat()
    }

    fn kept(arc: &str, numbers: &[usize]) -> Option<String> {
        let listed: Vec<String> = arc
            .lines()
            .filter_map(|line| line.strip_prefix(Self::ARC_DAYS))
            .filter_map(|listed| listed.strip_suffix(']'))
            .flat_map(|listed| listed.split(','))
            .filter(|listed| {
                listed
                    .trim()
                    .parse()
                    .is_ok_and(|number| numbers.contains(&number))
            })
            .map(|listed| listed.trim().to_owned())
            .collect();
        (!listed.is_empty()).then(|| {
            arc.lines()
                .map(|line| {
                    if line.starts_with(Self::ARC_DAYS) {
                        format!("{}{}]", Self::ARC_DAYS, listed.join(", "))
                    } else {
                        line.to_owned()
                    }
                })
                .collect::<Vec<String>>()
                .join("\n")
                + "\n"
        })
    }

    #[must_use]
    pub fn zone() -> TripZone {
        "+09:00".parse().expect("a written zone")
    }

    #[must_use]
    pub fn trip_text(arcs: &str, sources: &[DaySource<'_>]) -> String {
        let mut file: Table = arcs.parse().expect("fixture arcs");
        let head: Table = Self::HEAD.parse().expect("the fixture head");
        let days: Vec<Table> = sources
            .iter()
            .map(|source| source.day.parse().expect("a fixture day"))
            .collect();
        let numbers = days.iter().map(|day| &day[Self::NUMBER_KEY]);
        let routes: Vec<Value> = sources
            .iter()
            .zip(numbers)
            .map(|(source, number)| DaySource::numbered(source.routes, number))
            .collect();
        file.insert("trip".to_owned(), Value::Table(head));
        file.insert(
            "days".to_owned(),
            Value::Array(days.into_iter().map(Value::Table).collect()),
        );
        file.insert("routes".to_owned(), Value::Array(routes));
        toml::to_string(&file).expect("a fixture writes back")
    }

    pub fn parse(arcs: &str, sources: &[DaySource<'_>]) -> Result<Arc<Itinerary>, TripError> {
        Self::trip_text(arcs, sources)
            .parse::<Trip>()
            .map(|trip| Arc::clone(trip.itinerary()))
    }

    #[must_use]
    pub fn inspect(arcs: &str, sources: &[DaySource<'_>]) -> Vec<Finding> {
        TripSource::new(&Self::trip_text(arcs, sources)).findings()
    }
}
