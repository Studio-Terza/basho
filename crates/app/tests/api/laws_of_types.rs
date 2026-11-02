use std::cell::Cell;

use basho::app::session::progress_file::kept_progress::KeptProgress;
use basho::app::session::progress_file::kept_progress::legs_stamp::LegsStamp;
use basho::app::session::{DarkAfterSunset, Session};
use dioxus::prelude::*;
use guide::clock::Minutes;
use guide::guidance::LegProgress;
use guide::leg::LegIndex;
use laws::proptest::prelude::{Just, Strategy, any, prop_assert_eq};

use crate::ui_check::{Drawing, Stage};

const LEGS: std::ops::Range<usize> = 0..64;
const OFFSETS: std::ops::Range<i64> = -600..600;

type Compare = fn(&Session, &Session) -> bool;

thread_local! {
    static SHARED: Cell<bool> = const { Cell::new(false) };
    static COMPARE: Cell<Option<Compare>> = const { Cell::new(None) };
    static SAME: Cell<Option<bool>> = const { Cell::new(None) };
    static STAGE: Stage = Stage::all(&Drawing::Second.itinerary(), &Drawing::Second.wiki())
        .into_iter()
        .next()
        .expect("a stage");
}

struct SessionPair;

impl SessionPair {
    fn compared(shared: bool, compare: Compare) -> Option<bool> {
        SHARED.set(shared);
        COMPARE.set(Some(compare));
        let mut dom = VirtualDom::new(Self::render);
        dom.rebuild_in_place();
        SAME.take()
    }

    fn render() -> Element {
        let (itinerary, places) = (Drawing::Second.itinerary(), Drawing::Second.places());
        let stage = STAGE.with(Clone::clone);
        let overview = Drawing::Second.overview();
        let other = if SHARED.get() {
            Drawing::Second.itinerary()
        } else {
            Drawing::BuiltIn.itinerary()
        };
        let first = Session::staged(itinerary, places.clone(), overview.clone(), &stage);
        let second = Session::staged(other, places, overview, &stage);
        SAME.set(COMPARE.get().map(|compare| compare(&first, &second)));
        rsx! {}
    }
}

laws::law!(
    sessions_are_equal_when_they_show_one_itinerary,
    any::<bool>(),
    shared,
    {
        prop_assert_eq!(SessionPair::compared(shared, Session::eq), Some(shared));
    }
);

laws::roundtrip!(
    the_dark_setting_reads_back_from_its_text,
    any::<bool>().prop_map(|on| if on {
        DarkAfterSunset::On
    } else {
        DarkAfterSunset::Off
    }),
    DarkAfterSunset::to_string,
    |text: &String| text.parse::<DarkAfterSunset>().ok(),
    Some,
);

laws::roundtrip!(
    a_legs_stamp_reads_back_from_its_text,
    any::<u64>().prop_map(LegsStamp::new),
    LegsStamp::to_string,
    |text: &String| text.parse::<LegsStamp>().ok(),
    Some,
);

laws::roundtrip!(
    kept_progress_reads_back_from_its_file,
    (Just(Drawing::BuiltIn.itinerary()), LEGS, OFFSETS).prop_map(|(trip, leg, offset)| {
        let day = trip.days().first().expect("a day");
        KeptProgress::new(
            LegProgress::confirmed(day.number(), LegIndex::new(leg), Minutes::new(offset)),
            day,
        )
    }),
    KeptProgress::to_string,
    |text: &String| text.parse::<KeptProgress>().ok(),
    Some,
);
