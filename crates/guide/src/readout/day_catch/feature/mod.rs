mod feature_kind;
mod feature_record;

mod name_error;
use crate::catch_words::Readings;
use crate::readout::day_catch::FeatureId;
use crate::words::{Prose, Spelling};
use serde::Deserialize;

use crate::geo::{Coordinate, RouteFix, RouteLine};
use crate::ruby_text::RubyText;
use feature_kind::FeatureKind;
use feature_record::{FeatureRecord, NameForm};
use name_error::NameError;

#[derive(derive_more::AsRef, Clone, Debug, Deserialize)]
#[serde(try_from = "FeatureRecord")]
pub struct Feature {
    record: FeatureRecord,
    id: FeatureId,
    #[as_ref]
    name: RubyText,
    reference: Option<Prose>,
    lines: Vec<RouteLine>,
}

impl Feature {
    #[must_use]
    pub fn feature_id(&self) -> &FeatureId {
        &self.id
    }

    #[must_use]
    pub fn reference(&self) -> Option<&Prose> {
        self.reference.as_ref()
    }

    pub fn spot_names(&self) -> impl Iterator<Item = Spelling> + '_ {
        [NameForm::Written, NameForm::English]
            .into_iter()
            .filter_map(|form| self.record.name(form))
            .map(|name| Spelling::new(name.to_string()))
    }

    #[must_use]
    pub fn nearest_to(&self, here: Coordinate) -> Option<Coordinate> {
        self.lines
            .iter()
            .filter_map(|line| line.fix(here))
            .min_by_key(|fix| fix.off_route())
            .map(RouteFix::nearest)
    }
}

impl Feature {
    #[must_use]
    pub fn read_again(self, readings: &Readings) -> Feature {
        Feature::try_from(self.record.clone().read_with(readings)).unwrap_or(self)
    }
}

impl TryFrom<FeatureRecord> for Feature {
    type Error = NameError;

    fn try_from(record: FeatureRecord) -> Result<Feature, NameError> {
        let kind = record.kind();
        let annotated = record
            .name(NameForm::Written)
            .zip(record.name(NameForm::Kana))
            .map(|(name, kana)| format!("{{{name}|{kana}}}"));
        let road = record
            .reference()
            .filter(|_| kind == FeatureKind::Road)
            .map(|reference| format!("Road {}", reference.to_string().replace(';', " / ")));
        let name = [
            annotated,
            record.name(NameForm::Written).map(ToString::to_string),
            record.name(NameForm::English).map(ToString::to_string),
            road,
            Some(kind.phrase().to_string()),
        ]
        .into_iter()
        .flatten()
        .find_map(|text| RubyText::try_from(text).ok())
        .ok_or_else(|| NameError::from(record.feature_id().clone()))?;
        Ok(Feature {
            id: record.feature_id().clone(),
            name,
            reference: record.reference().cloned(),
            lines: record.geometry().lines(),
            record,
        })
    }
}
