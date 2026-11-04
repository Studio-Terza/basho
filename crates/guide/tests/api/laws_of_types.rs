use guide::clock::Minutes;
use guide::content::figure::Figure;
use guide::content::typed_part::{Measure, Price, ScheduledTrain, SpanUnit};
use guide::content::{DistanceUnit, SaidClock};
use guide::day_number::DayNumber;
use guide::english_name::EnglishName;
use guide::geo::Distance;
use guide::geo::route_line::route_fix::off_route::OffRoute;
use guide::guidance::LegProgress;
use guide::guidance::tracking::{Placing, Stage, Tracking};
use guide::leg::LegIndex;
use guide::plane::Pixels;
use guide::words::Spelling;
use laws::proptest::prelude::{Strategy, any, prop_assert, prop_assert_eq};

const FINITE: std::ops::Range<f64> = -1.0e9..1.0e9;
const LEGS: std::ops::Range<usize> = 0..64;
const OFFSETS: std::ops::Range<i64> = -600..600;
const HOURS: std::ops::Range<u8> = 0..24;
const MINUTES: std::ops::Range<u8> = 0..60;
const FIGURES: &str = "[0-9]{1,3}(,[0-9]{3}){0,2}(\\.[0-9]{1,2})?";
const STAGES: [Stage; 3] = [Stage::Boarding, Stage::Under, Stage::Finished];

laws::roundtrip!(
    a_day_number_reads_back_from_json,
    any::<u8>().prop_map(DayNumber::new),
    |day: &DayNumber| serde_json::to_string(day).unwrap(),
    |text: &String| serde_json::from_str::<DayNumber>(text).ok(),
    Some,
);

laws::roundtrip!(
    an_english_name_reads_back_from_json,
    any::<String>().prop_map(EnglishName::new),
    |name: &EnglishName| serde_json::to_string(name).unwrap(),
    |text: &String| serde_json::from_str::<EnglishName>(text).ok(),
    Some,
);

laws::roundtrip!(
    kept_progress_reads_back_from_its_line,
    (any::<u8>(), LEGS, OFFSETS).prop_map(|(day, leg, offset)| {
        LegProgress::confirmed(
            DayNumber::new(day),
            LegIndex::new(leg),
            Minutes::new(offset),
        )
    }),
    LegProgress::to_string,
    |line: &String| line.parse::<LegProgress>().ok(),
    Some,
);

laws::roundtrip!(
    a_manual_set_reads_back_from_its_words,
    (LEGS, 0..STAGES.len(), LEGS, 0..STAGES.len()).prop_map(
        |(floor, floor_stage, base, base_stage)| {
            Tracking::pinned(
                Placing::new(LegIndex::new(floor), STAGES[floor_stage]),
                Placing::new(LegIndex::new(base), STAGES[base_stage]),
            )
        }
    ),
    Tracking::to_string,
    |words: &String| words.parse::<Tracking>().ok(),
    Some,
);

laws::law!(
    distances_order_as_their_meters_do,
    (FINITE, FINITE),
    meters,
    {
        let (left, right) = meters;
        prop_assert_eq!(
            Distance::new(left).partial_cmp(&Distance::new(right)),
            left.partial_cmp(&right)
        );
    }
);

laws::law!(
    pixels_order_as_their_numbers_do,
    (FINITE, FINITE),
    numbers,
    {
        let (left, right) = numbers;
        prop_assert_eq!(
            Pixels::new(left).partial_cmp(&Pixels::new(right)),
            left.partial_cmp(&right)
        );
    }
);

laws::law!(
    off_route_compares_as_its_distance_does,
    (FINITE, FINITE),
    meters,
    {
        let (left, right) = meters;
        let away = OffRoute::new(Distance::new(left));
        prop_assert!(away == Distance::new(left));
        prop_assert_eq!(
            away.partial_cmp(&Distance::new(right)),
            left.partial_cmp(&right)
        );
    }
);

laws::roundtrip!(
    a_spelling_reads_back_from_json,
    any::<String>().prop_map(Spelling::new),
    |spelling: &Spelling| serde_json::to_string(spelling).unwrap(),
    |text: &String| serde_json::from_str::<Spelling>(text).ok(),
    Some,
);

laws::roundtrip!(
    a_said_clock_reads_back_from_its_words,
    laws::proptest::prop_oneof![
        (HOURS, MINUTES).prop_map(|(hours, minutes)| format!("{hours}:{minutes:02}")),
        (HOURS, MINUTES).prop_map(|(hours, minutes)| format!("{hours:02}:{minutes:02}")),
    ]
    .prop_filter_map("a said clock", |words| words.parse::<SaidClock>().ok()),
    SaidClock::to_string,
    |words: &String| words.parse::<SaidClock>().ok(),
    Some,
);

laws::roundtrip!(
    a_figure_reads_back_from_its_digits,
    FIGURES.prop_filter_map("a figure", |digits| digits.parse::<Figure>().ok()),
    Figure::to_string,
    |digits: &String| digits.parse::<Figure>().ok(),
    Some,
);

laws::roundtrip!(
    a_price_reads_back_from_its_yen,
    FIGURES
        .prop_map(|digits| format!("¥{digits}"))
        .prop_filter_map("a price", |words| words.parse::<Price>().ok()),
    Price::to_string,
    |words: &String| words.parse::<Price>().ok(),
    Some,
);

laws::roundtrip!(
    a_train_reads_back_from_its_name_and_number,
    (
        laws::proptest::prop_oneof!["Hayabusa", "Yamabiko", "Komachi", "Tsubasa", "Inaho"],
        FIGURES,
    )
        .prop_map(|(brand, number)| format!("{brand} {number}"))
        .prop_filter_map("a train", |words| words.parse::<ScheduledTrain>().ok()),
    ScheduledTrain::to_string,
    |words: &String| words.parse::<ScheduledTrain>().ok(),
    Some,
);

laws::roundtrip!(
    a_distance_reads_back_from_its_words,
    (
        FIGURES,
        laws::proptest::prop_oneof![" ", "-", ""],
        laws::proptest::prop_oneof!["km", "m"],
    )
        .prop_map(|(figure, joint, unit)| format!("{figure}{joint}{unit}"))
        .prop_filter_map("a distance", |words| {
            words.parse::<Measure<DistanceUnit>>().ok()
        }),
    Measure::<DistanceUnit>::to_string,
    |words: &String| words.parse::<Measure<DistanceUnit>>().ok(),
    Some,
);

laws::roundtrip!(
    a_span_reads_back_from_its_words,
    (
        FIGURES,
        laws::proptest::prop_oneof![" ", "-", ""],
        laws::proptest::prop_oneof!["min", "minute", "minutes", "hour", "hours"],
    )
        .prop_map(|(figure, joint, unit)| format!("{figure}{joint}{unit}"))
        .prop_filter_map("a span", |words| words.parse::<Measure<SpanUnit>>().ok()),
    Measure::<SpanUnit>::to_string,
    |words: &String| words.parse::<Measure<SpanUnit>>().ok(),
    Some,
);
