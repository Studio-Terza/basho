use guide::clock::Minutes;
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
