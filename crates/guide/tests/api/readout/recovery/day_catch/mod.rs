mod feature;
mod leg_catch;

use guide::geo::Coordinate;
use guide::geo::Degrees;
use guide::leg::LegIndex;
use guide::reading::Reading;
use guide::readout::DayCatch;

const FIXTURE: &str = r#"{"day":3,"features":[
    {"id":"f1","kind":"road","name":"Road A","ref":"28","geometry":{"type":"LineString","coordinates":[[140.0,38.0],[140.0,38.01]]}},
    {"id":"f2","kind":"river","name":"River B","geometry":{"type":"Point","coordinates":[140.002,38.0]}},
    {"id":"f3","kind":"peak","name":"Peak C","geometry":{"type":"Point","coordinates":[140.03,38.0]}},
    {"id":"f4","kind":"lake","name":"Lake D","geometry":{"type":"Point","coordinates":[140.001,38.0]}}],
    "legs":[{"index":1,"catch":[{"feature":"f3","ste":["The peak is far."]},{"feature":"f1","ste":["The road goes north."]},{"feature":"f2","ste":["The river is east."]}]}]}"#;
const HERE: Coordinate = Coordinate::new(Degrees::new(38.005), Degrees::new(140.0005));

struct Toward;

impl Toward {
    #[expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "laws::table! hands each row by reference"
    )]
    fn names(leg: &usize) -> Vec<String> {
        let catch: DayCatch = serde_json::from_str(FIXTURE).unwrap();
        catch
            .toward(HERE, LegIndex::new(*leg), None)
            .iter()
            .map(|sight| sight.name().text(Reading::Written).to_string())
            .collect()
    }
}

laws::table!(
    the_two_nearest_features_picked_for_the_leg_come_first_and_a_leg_with_no_picks_uses_the_day,
    Toward::names,
    [
        1 => ["Road A", "River B"],
        0 => ["Road A", "Lake D"],
    ],
);
