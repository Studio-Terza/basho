use serde_json::Value;

const STYLE: &str = include_str!("../../../assets/map_style.json");
const WAYS: [(&str, &str); 13] = [
    ("highway", "paved"),
    ("highway", "unpaved"),
    ("major", "paved"),
    ("major", "unpaved"),
    ("minor", "paved"),
    ("minor", "unpaved"),
    ("street", "paved"),
    ("street", "unpaved"),
    ("lane", "paved"),
    ("lane", "unpaved"),
    ("track", "paved"),
    ("track", "unpaved"),
    ("trail", "paved"),
];
const TRAIL_SURFACE: (&str, &str) = ("trail", "trail");
const UNPAVED_LAYER: &str = "road-unpaved";
const ROAD_LAYER: &str = "road";

struct Tags<'tags> {
    class: &'tags str,
    surface: &'tags str,
}

impl Tags<'_> {
    fn value(&self, key: &str) -> Option<&str> {
        match key {
            "class" => Some(self.class),
            "surface" => Some(self.surface),
            _ => None,
        }
    }

    fn drawn(&self) -> Vec<String> {
        let style: Value = serde_json::from_str(STYLE).unwrap();
        style["layers"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|layer| layer["source-layer"] == ROAD_LAYER && layer["type"] == "line")
            .filter(|layer| self.held(&layer["filter"]))
            .map(|layer| layer["id"].as_str().unwrap().to_owned())
            .filter(|id| !id.starts_with("road-bridge"))
            .collect()
    }

    fn held(&self, filter: &Value) -> bool {
        let Some([operator, rest @ ..]) = filter.as_array().map(Vec::as_slice) else {
            return false;
        };
        let key = |expression: &Value| {
            expression
                .get(1)
                .and_then(Value::as_str)
                .and_then(|key| self.value(key))
        };
        match (operator.as_str(), rest) {
            (Some("all"), filters) => filters.iter().all(|inner| self.held(inner)),
            (Some("=="), [lookup, wanted]) => key(lookup) == wanted.as_str(),
            (Some("!="), [lookup, wanted]) => key(lookup) != wanted.as_str(),
            (Some("in"), [lookup, list]) => list
                .get(1)
                .and_then(Value::as_array)
                .is_some_and(|names| names.iter().any(|name| key(lookup) == name.as_str())),
            _ => false,
        }
    }
}

#[test]
fn every_kind_of_road_the_tiles_carry_is_drawn_and_an_unpaved_one_only_dashed() {
    let mut undrawn = Vec::new();
    let mut solid_unpaved = Vec::new();
    for (class, surface) in WAYS.into_iter().chain([TRAIL_SURFACE]) {
        let layers = Tags { class, surface }.drawn();
        if layers.is_empty() {
            undrawn.push((class, surface));
        }
        if surface == "unpaved" && layers != [UNPAVED_LAYER] {
            solid_unpaved.push((class, layers));
        }
    }
    assert_eq!(undrawn, Vec::<(&str, &str)>::new());
    assert_eq!(solid_unpaved, Vec::<(&str, Vec<String>)>::new());
}
