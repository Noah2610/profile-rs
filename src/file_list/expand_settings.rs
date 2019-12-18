#[derive(Default, Builder)]
#[builder(setter(into))]
pub struct ExpandSettings {
    #[builder(default)]
    pub recurse: bool,
}

impl ExpandSettings {
    pub fn builder() -> ExpandSettingsBuilder {
        ExpandSettingsBuilder::default()
    }
}
