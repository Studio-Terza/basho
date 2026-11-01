use guide::clock::Minutes;

#[test]
fn minutes_read_as_hours_and_minutes() {
    assert_eq!(Minutes::new(62).to_string(), "1 h 2 min");
    assert_eq!(Minutes::new(-60).to_string(), "-1 h");
}

const REFUSED: [f64; 4] = [f64::INFINITY, f64::NEG_INFINITY, f64::NAN, 1e19];
const TIES: [(f64, i64); 3] = [(2.5, 2), (3.5, 4), (-1.5, -2)];

#[test]
fn rounding_refuses_counts_outside_i64_and_breaks_ties_to_even() {
    let refused: Vec<Option<Minutes>> = REFUSED.map(Minutes::rounded).to_vec();
    let ties: Vec<Option<Minutes>> = TIES.map(|(count, _)| Minutes::rounded(count)).to_vec();
    assert_eq!(refused, vec![None; REFUSED.len()]);
    assert_eq!(
        ties,
        TIES.map(|(_, whole)| Some(Minutes::new(whole))).to_vec()
    );
}
