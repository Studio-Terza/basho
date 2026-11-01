use strum::IntoStaticStr;

#[derive(Clone, Copy, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
pub enum XcrunTool {
    Altool,
    Actool,
}
