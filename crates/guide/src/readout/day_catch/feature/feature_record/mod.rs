mod geometry;
mod name_form;

use crate::catch_words::{FeatureName, Readings};
use crate::readout::day_catch::FeatureId;
use crate::words::Prose;
use serde::Deserialize;
use serde::de::IgnoredAny;

use super::feature_kind::FeatureKind;
use geometry::Geometry;
pub use name_form::NameForm;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeatureRecord {
    #[expect(dead_code, reason = "import detail, kept only so the file loads")]
    class: Option<IgnoredAny>,
    #[expect(dead_code, reason = "import detail, kept only so the file loads")]
    highway: Option<IgnoredAny>,
    #[expect(dead_code, reason = "import detail, kept only so the file loads")]
    osm: Option<IgnoredAny>,
    #[expect(dead_code, reason = "import detail, kept only so the file loads")]
    span_m: Option<IgnoredAny>,
    #[expect(dead_code, reason = "import detail, kept only so the file loads")]
    ele: Option<IgnoredAny>,
    #[expect(dead_code, reason = "import detail, kept only so the file loads")]
    operator: Option<IgnoredAny>,
    #[expect(dead_code, reason = "import detail, kept only so the file loads")]
    other_names: Option<IgnoredAny>,
    id: FeatureId,
    kind: FeatureKind,
    name: Option<FeatureName>,
    name_kana: Option<FeatureName>,
    name_en: Option<FeatureName>,
    #[serde(rename = "ref")]
    reference: Option<Prose>,
    geometry: Geometry,
}

impl FeatureRecord {
    #[must_use]
    pub fn feature_id(&self) -> &FeatureId {
        &self.id
    }

    #[must_use]
    pub fn kind(&self) -> FeatureKind {
        self.kind
    }

    #[must_use]
    pub fn name(&self, form: NameForm) -> Option<&FeatureName> {
        match form {
            NameForm::Written => self.name.as_ref(),
            NameForm::Kana => self.name_kana.as_ref(),
            NameForm::English => self.name_en.as_ref(),
        }
    }

    #[must_use]
    pub fn reference(&self) -> Option<&Prose> {
        self.reference.as_ref()
    }

    #[must_use]
    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }
}

impl FeatureRecord {
    #[must_use]
    pub fn read_with(self, readings: &Readings) -> FeatureRecord {
        let name_kana = self.name_kana.clone().or_else(|| {
            self.name
                .as_ref()
                .and_then(|name| readings.get(name))
                .cloned()
        });
        FeatureRecord { name_kana, ..self }
    }
}
