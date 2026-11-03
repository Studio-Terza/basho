use osm_items::OsmLine;

const SPOT: &str = r#"{"type":"Point","coordinates":[140.0,38.0]}"#;

struct Item;

impl Item {
    fn tags(geometry: &str, tags: &str) -> Vec<String> {
        Self::parsed(geometry, tags)
            .tile_item()
            .expect("the item is kept")
            .drawn()
            .tags()
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    fn parsed(geometry: &str, tags: &str) -> OsmLine {
        format!(r#"{{"type":"Feature","geometry":{geometry},"properties":{tags}}}"#)
            .parse()
            .expect("the line is JSON")
    }
}

#[test]
fn a_road_keeps_its_rank_and_surface() {
    assert_eq!(
        Item::tags(
            r#"{"type":"LineString","coordinates":[[140.0,38.0],[140.01,38.01]]}"#,
            r#"{"highway":"secondary","surface":"gravel","bridge":"yes"}"#,
        ),
        ["class=major", "surface=unpaved", "structure=bridge"]
    );
}

impl Item {
    fn spot_tags(tags: &&str) -> Vec<String> {
        Item::tags(SPOT, tags)
    }
}

laws::table!(
    a_latin_name_wins_a_kanji_name_shows_its_osm_reading_else_a_marked_generated_one_and_kana_stands_alone,
    Item::spot_tags,
    [
        r#"{"shop":"convenience","name":"山田","brand:en":"Lawson"}"# => ["class=konbini", "label=Lawson"],
        r#"{"shop":"convenience","name":"山田商店","name:ja-Hira":"やまだしょうてん"}"# => ["class=konbini", "label=山田商店", "reading=やまだしょうてん"],
        r#"{"shop":"convenience","name":"山田商店","name:ja_kana":"ヤマダショウテン"}"# => ["class=konbini", "label=山田商店", "reading=やまだしょうてん"],
        r#"{"shop":"convenience","name":"山田商店"}"# => ["class=konbini", "label=山田商店", "guess=やまだしょうてん"],
        r#"{"shop":"convenience","name":"山田商店","name:ja-Hira":"山田しょうてん"}"# => ["class=konbini", "label=山田商店", "guess=やまだしょうてん"],
        r#"{"place":"village","name":"山田"}"# => ["class=village", "label=山田", "guess=やまだ"],
        r#"{"shop":"convenience","name":"ローソン"}"# => ["class=konbini", "label=ローソン"],
    ],
);

#[test]
fn items_nobody_draws_are_dropped() {
    assert!(
        Item::parsed(SPOT, r#"{"highway":"traffic_signals"}"#)
            .tile_item()
            .is_none()
    );
}

#[test]
fn bridge_no_and_tunnel_no_draw_no_structure() {
    assert_eq!(
        Item::tags(
            r#"{"type":"LineString","coordinates":[[140.0,38.0],[140.01,38.01]]}"#,
            r#"{"highway":"service","bridge":"no","tunnel":"no"}"#,
        ),
        ["class=lane", "surface=paved"]
    );
    assert_eq!(
        Item::tags(
            r#"{"type":"LineString","coordinates":[[140.0,38.0],[140.01,38.01]]}"#,
            r#"{"highway":"footway","bridge":"no","tunnel":"yes"}"#,
        ),
        ["class=trail", "surface=trail", "structure=tunnel"]
    );
}

#[test]
fn a_wood_is_drawn_as_forest_land() {
    assert_eq!(
        Item::tags(
            r#"{"type":"MultiPolygon","coordinates":[[[[140.0,38.0],[140.01,38.0],[140.01,38.01],[140.0,38.0]]]]}"#,
            r#"{"natural":"wood"}"#,
        ),
        ["class=forest"]
    );
}
