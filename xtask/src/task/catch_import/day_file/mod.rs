use std::str::FromStr;

use guide::readout::recovery::catch_words::{FeatureName, Readings};
use guide::trip::KnownNames;
use newtype::Newtype;
use serde::Serialize;
use serde_json::Value;

pub mod geo_json;
pub mod note_text;

use geo_json::GeoJson;
use note_text::NoteText;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct DayFile {
    json: GeoJson,
}

impl DayFile {
    pub fn feature_readings(&self) -> Readings {
        Readings::new(
            self.json
                .json()
                .get("features")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|feature| {
                    Some((
                        FeatureName::new(feature.get("name")?.as_str()?.to_owned()),
                        FeatureName::new(feature.get("name_kana")?.as_str()?.to_owned()),
                    ))
                })
                .collect(),
        )
    }

    #[must_use]
    pub fn without_nulls(self) -> DayFile {
        DayFile {
            json: self.json.without_nulls(),
        }
    }

    pub fn sourced_over(&self, readings: Readings) -> Readings {
        let sourced = self
            .json
            .json()
            .get("readings")
            .and_then(Value::as_object)
            .into_iter()
            .flatten()
            .filter(|(_, reading)| {
                reading
                    .get("source")
                    .and_then(Value::as_str)
                    .is_some_and(|source| !source.is_empty())
            })
            .filter_map(|(name, reading)| {
                Some((
                    FeatureName::new(name.clone()),
                    FeatureName::new(reading.get("kana")?.as_str()?.to_owned()),
                ))
            });
        Readings::new(readings.into_inner().into_iter().chain(sourced).collect())
    }

    #[must_use]
    pub fn annotated(self, readings: &Readings, names: &KnownNames) -> DayFile {
        let parted = |text: &str| {
            NoteText::from(text)
                .annotated(readings)
                .parts(names)
                .and_then(|parts| serde_json::to_value(parts).ok())
        };
        let mut json = self.json.into_value();
        json.get_mut("legs")
            .and_then(Value::as_array_mut)
            .into_iter()
            .flatten()
            .filter_map(|leg| leg.get_mut("catch").and_then(Value::as_array_mut))
            .flatten()
            .filter_map(|note| note.get_mut("ste"))
            .for_each(|ste| {
                *ste = ste.as_str().and_then(parted).unwrap_or_else(|| ste.clone());
            });
        DayFile {
            json: GeoJson::new(json),
        }
    }
}

impl FromStr for DayFile {
    type Err = serde_json::Error;

    fn from_str(text: &str) -> Result<DayFile, serde_json::Error> {
        serde_json::from_str(text).map(|json| DayFile { json })
    }
}
