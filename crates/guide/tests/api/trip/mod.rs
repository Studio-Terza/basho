use guide::english_name::EnglishName;
use std::ops::Range;
use std::path::Path;

use guide::Trip;
use guide::clock::ClockTime;
use guide::clock::Sunset;
use guide::geo::{Coordinate, Degrees, RouteLine};

const SAMPLE: &str = include_str!("../../../../../data/trips/sample.toml");
const NO_SUCH_FILE: &str = "no/such/trip.toml";
const SUNSET_HOUR: u16 = 20;
const WITHIN_THE_HOUR: Range<i64> = 0..60;
const UNMAPPED_END: &str = "{ no_mapped_path = { via = [[-77.988, 38.0]] } }";
const GAP_METERS: Range<f64> = 80.0..90.0;
const PART_WAY: Coordinate = Coordinate::new(Degrees::new(38.0), Degrees::new(-77.987));
const SPOILED: [[&str; 3]; 28] = [
    ["name = \"Sample\"", "name = ", "line 2"],
    [
        "utc_offset = \"-04:00\"",
        "utc_offset = \"Tuesday\"",
        "\"Tuesday\" is not a UTC offset",
    ],
    ["lang = \"en\"\n", "", "missing field `lang`"],
    [
        "lang = \"en\"",
        "lang = \"Klingon\"",
        "unknown variant `Klingon`",
    ],
    [
        "number = 2\ndate = \"2027-05-04\"",
        "number = 2\ndate = \"2027-05-03\"",
        "day 2: day numbers",
    ],
    ["number = 2\n", "number = 1\n", "day 1: day numbers"],
    [
        "title = [{ entry = \"Mill Inn\" }, \" → Oakford\"]",
        "titel = [{ entry = \"Mill Inn\" }, \" → Oakford\"]",
        "unknown field `titel`",
    ],
    [
        "place = \"eastfield\"",
        "place = \"nowhere\"",
        "day 1 leg 1: place \"nowhere\"",
    ],
    [
        "1 = { segments = [{ mapped = { nodes = [201, 202] } }] }",
        "",
        "day 2 leg 1: has no line",
    ],
    [
        "[\"The old oak\"]",
        "[\"The new oak\"]",
        "\"The new oak\" isn't a starred point",
    ],
    [
        "handrail = [\"The stream is on your left.\"]",
        "handrial = \"The stream is on your left.\"",
        "unknown field `handrial`",
    ],
    [
        "check_in_by = ",
        "check_in_byy = ",
        "unknown field `check_in_byy`",
    ],
    [
        "[landmarks]\n",
        "[landmarks]\npeeks = []\n",
        "unknown field `peeks`",
    ],
    ["km = 0.35", "km = nan", "is not a finite number"],
    [
        "[[days]]\nnumber = 1",
        "[[places]]\nslug = \"eastfield\"\nname = \"Eastfield\"\nenglish = \"Eastfield\"\ntagline = [\"Again.\"]\nmeaning = [\"Again.\"]\nexcerpts = []\ndisagreements = []\n\n[places.origin]\ntext = [\"Again.\"]\nevidence = { unverified = [\"Again.\"] }\n\n[places.geography]\ntext = [\"Again.\"]\nevidence = { unverified = [\"Again.\"] }\n\n[[days]]\nnumber = 1",
        "place \"eastfield\" is listed more than once",
    ],
    [
        "[[places]]\nslug",
        "[[arcs]]\nnumber = 1\ntitle = [\"Again\"]\nstart = \"Here\"\ndays = []\n\n[[places]]\nslug",
        "arc 1 is listed more than once",
    ],
    [
        "1 = { segments = [{ mapped = { nodes = [101, 102, 103] } }] }",
        "1 = { segments = [{ mapped = { nodes = [101, 102, 103] } }] }\n9 = { coords = [[-77.9, 38.0], [-77.8, 38.0]] }",
        "day 1 leg 9: a [[routes]] line is keyed to a leg",
    ],
    [
        "date = \"2027-05-04\"",
        "date = \"2027-05-05\"",
        "day 2: day numbers and dates must rise",
    ],
    [
        "[[routes]]\nday = 1",
        "[[landmarks.stars]]\nname = \"The new oak\"\nenglish = \"The new oak\"\nshort = \"The old oak\"\nday = 2\nwhat = [\"Another tree.\"]\nat = { lat = 38.0, lon = -77.983 }\nlocated = { url = \"https://example.org\", checked = \"2026-10-31\" }\nsource = { url = \"https://example.org\", checked = \"2026-10-31\" }\n\n[[routes]]\nday = 1",
        "day 2: two starred points share the short name \"The old oak\"",
    ],
    [
        "nodes = [101, 102, 103]",
        "nodes = [101, 103]",
        "day 1 leg 1: no way joins node 101 to node 103",
    ],
    [
        "nodes = [101, 102, 103]",
        "nodes = [101, 102, 999]",
        "day 1 leg 1: node 999 is on no way",
    ],
    [
        "nodes = [101, 102, 103]",
        "nodes = [101]",
        "day 1 leg 1: a mapped segment needs two nodes",
    ],
    [
        "{ mapped = { nodes = [101, 102, 103] } }",
        "{ mapped = { nodes = [101, 102] } }, { mapped = { nodes = [103, 104] } }",
        "day 1 leg 1: two segments do not meet",
    ],
    [
        "{ nodes = [201, 202] }",
        "{ nodes = [201, 202], from = [-77.98, 38.5] }",
        "day 2 leg 1: a segment end is ",
    ],
    [
        "1 = { segments = [{ mapped = { nodes = [201, 202] } }] }",
        "1 = { coords = [[-77.988, 38.0], [-77.980, 38.0]] }",
        "day 2 leg 1: a walking line needs `segments`",
    ],
    [
        "{ id = 2, nodes = [[103, -77.996, 38.0], [104, -77.990, 38.0]] },\n]\n[routes.legs]\n1 = { segments = [{ mapped = { nodes = [101, 102, 103] } }] }\n2 = { segments = [{ mapped = { nodes = [103, 104] } }, { no_mapped_path = { via = [[-77.988, 38.0]] } }] }",
        "{ id = 2, nodes = [[103, -77.996, 38.0], [110, -77.992, 38.0]] },\n{ id = 5, nodes = [[111, -77.993, 37.999], [112, -77.993, 38.001], [104, -77.990, 38.0]] },\n]\n[routes.legs]\n1 = { segments = [{ mapped = { nodes = [101, 102, 103] } }] }\n2 = { segments = [{ mapped = { nodes = [103, 110], to = [-77.993, 38.0] } }, { mapped = { nodes = [111, 112, 104], from = [-77.993, 38.0] } }, { no_mapped_path = { via = [[-77.988, 38.0]] } }] }",
        "day 1 leg 2: two segments do not meet",
    ],
    [
        "1 = { segments = [{ mapped = { nodes = [101, 102, 103] } }] }",
        "1 = { typo = 7, segments = [{ mapped = { nodes = [101, 102, 103] } }] }",
        "did not match any variant of untagged enum LegEntry",
    ],
    [
        "1 = { segments = [{ mapped = { nodes = [101, 102, 103] } }] }",
        "1 = { coords = [[0.0, 0.0]], segments = [{ mapped = { nodes = [101, 102, 103] } }] }",
        "did not match any variant of untagged enum LegEntry",
    ],
];

#[test]
fn the_trip_file_alone_sets_the_name_and_the_clock_of_the_trip() {
    let loaded = SAMPLE.parse::<Trip>().expect("the sample trip loads");
    let trip = loaded.itinerary();
    let sunset = trip.days()[0].sunset().and_then(Sunset::time);
    let past_the_hour = sunset.map(|set| ClockTime::at_hour(SUNSET_HOUR).minutes_until(set));
    assert_eq!(
        AsRef::<EnglishName>::as_ref(trip.head()).to_string(),
        "Sample"
    );
    assert!(past_the_hour.is_some_and(|past| WITHIN_THE_HOUR.contains(&past.count())));
}

#[test]
fn each_bad_file_fails_at_load_and_says_what_is_wrong() {
    let unnamed: Vec<&str> = SPOILED
        .iter()
        .filter(|[good, bad, said]| {
            !SAMPLE
                .replacen(good, bad, 1)
                .parse::<Trip>()
                .map_or_else(|error| error.to_string(), |_| String::new())
                .contains(said)
        })
        .map(|[good, ..]| *good)
        .collect();
    assert_eq!(unnamed, Vec::<&str>::new());
}

#[test]
fn only_a_stretch_declared_unmapped_is_a_no_mapped_path() {
    let loaded = SAMPLE.parse::<Trip>().expect("the sample trip loads");
    let unmapped: Vec<usize> = loaded.itinerary().days()[0]
        .legs()
        .iter()
        .map(|leg| leg.no_mapped_paths().count())
        .collect();
    assert_eq!(unmapped, [0, 1]);
}

#[test]
fn a_no_mapped_path_reaches_from_the_segment_before_to_the_segment_after() {
    let joined = SAMPLE
        .replacen(
            UNMAPPED_END,
            "{ no_mapped_path = { via = [] } }, { mapped = { nodes = [105, 106] } }",
            1,
        )
        .replacen(
            "[104, -77.990, 38.0]] }",
            "[104, -77.990, 38.0], [105, -77.989, 38.0], [106, -77.988, 38.0]] }",
            1,
        )
        .parse::<Trip>()
        .expect("the joined trip loads");
    let leg = &joined.itinerary().days()[0].legs()[1];
    let gaps: Vec<f64> = leg
        .no_mapped_paths()
        .map(|stretch| AsRef::<RouteLine>::as_ref(stretch).length().meters())
        .collect();
    assert!(gaps.iter().all(|gap| GAP_METERS.contains(gap)), "{gaps:?}");
    assert_eq!(gaps.len(), 1);
}

#[test]
fn a_mapped_segment_may_start_part_way_along_its_first_way() {
    let part_way = SAMPLE
        .replacen(
            "{ nodes = [201, 202] }",
            "{ nodes = [201, 202], from = [-77.987, 38.0] }",
            1,
        )
        .parse::<Trip>();
    assert!(part_way.is_ok_and(|trip| {
        trip.itinerary().days()[1].legs()[0]
            .line()
            .points()
            .first()
            .is_some_and(|start| *start == PART_WAY)
    }));
}

#[test]
fn a_file_that_is_not_there_is_named_in_the_error() {
    let error = Trip::load(Path::new(NO_SUCH_FILE)).map(|_| ());
    assert!(format!("{error:?}").contains(NO_SUCH_FILE));
}

#[test]
fn a_time_without_its_fact_or_plan_mark_fails_at_load() {
    let bare = SAMPLE.replace("start = { plan = \"09:00\" }", "start = \"09:00\"");
    assert!(bare.parse::<Trip>().is_err());
}
