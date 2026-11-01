use laws::proptest::prelude::{BoxedStrategy, Strategy, any};
use report::{Capture, Fact, Report, ReportNumber, ReportText};

const NUMBERS: std::ops::Range<usize> = 0..100_000;
const TEXT: &str = "[ -~\n±·]{0,12}";
const MOST_DECISIONS: usize = 3;
const FACTS: [Fact; 4] = [Fact::Screen, Fact::Step, Fact::Position, Fact::Tracker];

struct Made;

impl Made {
    fn text() -> BoxedStrategy<ReportText> {
        TEXT.prop_map(ReportText::new).boxed()
    }

    fn capture() -> BoxedStrategy<Capture> {
        (
            Self::text(),
            Self::text(),
            any::<bool>(),
            laws::proptest::collection::vec(Self::text(), 0..MOST_DECISIONS),
        )
            .prop_map(|(time, fact, known, decisions)| {
                Capture::new(time)
                    .knowing(Fact::Screen, Some(fact.clone()))
                    .knowing(Fact::Tracker, known.then_some(fact))
                    .after_decisions(decisions)
            })
            .boxed()
    }

    fn report() -> BoxedStrategy<Report> {
        (NUMBERS, Self::capture(), Self::text())
            .prop_map(|(number, capture, note)| {
                Report::new(ReportNumber::new(number), capture).noted(note)
            })
            .boxed()
    }
}

laws::roundtrip!(
    a_report_number_reads_back_from_json,
    NUMBERS.prop_map(ReportNumber::new),
    |number: &ReportNumber| serde_json::to_string(number).unwrap(),
    |text: &String| serde_json::from_str::<ReportNumber>(text).ok(),
    Some,
);

laws::roundtrip!(
    report_text_reads_back_from_json,
    Made::text(),
    |text: &ReportText| serde_json::to_string(text).unwrap(),
    |json: &String| serde_json::from_str::<ReportText>(json).ok(),
    Some,
);

laws::roundtrip!(
    report_text_reads_back_from_its_display,
    Made::text(),
    ReportText::to_string,
    |shown: &String| shown.parse::<ReportText>().ok(),
    Some,
);

laws::roundtrip!(
    a_capture_reads_back_from_toml,
    Made::capture(),
    |capture: &Capture| toml::to_string(capture).unwrap(),
    |text: &String| toml::from_str::<Capture>(text).ok(),
    Some,
);

laws::roundtrip!(
    a_report_reads_back_from_toml,
    Made::report(),
    |report: &Report| toml::to_string(report).unwrap(),
    |text: &String| toml::from_str::<Report>(text).ok(),
    Some,
);

laws::roundtrip!(
    a_fact_reads_back_from_json,
    (0..FACTS.len()).prop_map(|at| FACTS[at]),
    |fact: &Fact| serde_json::to_string(fact).unwrap(),
    |json: &String| serde_json::from_str::<Fact>(json).ok(),
    Some,
);
