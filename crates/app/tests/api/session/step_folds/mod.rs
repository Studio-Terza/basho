use guide::itinerary::DayNumber;

use basho::app::history::Fold;
use basho::app::history::StepFolds;
use basho::app::step::StepNumber;
use basho::app::step::StepState;

const DAY: DayNumber = DayNumber::new(4);
const OTHER_DAY: DayNumber = DayNumber::new(5);
const STEP: StepNumber = StepNumber::from_position(1);

#[test]
fn the_current_step_starts_open_and_a_tap_folds_it_in_place() {
    let folds = StepFolds::default();
    let tapped = folds.toggled(DAY, STEP);
    let again = tapped.toggled(DAY, STEP);
    assert_eq!(
        [folds, tapped, again].map(|each| each.fold(DAY, STEP, StepState::Current)),
        [Fold::Open, Fold::Closed, Fold::Open]
    );
}

#[test]
fn opening_a_step_on_another_day_forgets_the_old_day() {
    let folds = StepFolds::default()
        .toggled(DAY, STEP)
        .toggled(OTHER_DAY, STEP);
    assert_eq!(
        [DAY, OTHER_DAY].map(|day| folds.fold(day, STEP, StepState::Later)),
        [Fold::Closed, Fold::Open]
    );
}
