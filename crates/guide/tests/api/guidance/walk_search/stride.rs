use crate::fixture::Fixture;
use crate::walker::Walker;
use guide::clock::Minutes;
use guide::geo::Distance;
use guide::guidance::LegProgress;
use guide::leg::LegIndex;

const HILL_DAY: usize = 3;
const ZIGZAG: LegIndex = LegIndex::new(1);
const SIDEWAYS: [Distance; 2] = [Distance::new(-35.0), Distance::new(35.0)];
const ONE_IN: usize = 20;

#[test]
fn with_sideways_gps_error_on_switchbacks_the_arrow_is_right_nineteen_times_in_twenty() {
    let trip = Fixture::bundled_days(&[HILL_DAY]);
    let day = trip.day(HILL_DAY);
    let wrong: Vec<String> = SIDEWAYS
        .iter()
        .filter_map(|aside| {
            let walker = Walker::kept(
                day,
                LegProgress::confirmed(day.number(), ZIGZAG, Minutes::ZERO),
            );
            let fixes = walker.fixes_on(ZIGZAG);
            let turns = walker.wrong_turns(ZIGZAG, Minutes::ZERO, *aside, Distance::default());
            (turns.len() * ONE_IN > fixes).then(|| format!("{turns:?}"))
        })
        .collect();
    assert_eq!(wrong, Vec::<String>::new());
}
