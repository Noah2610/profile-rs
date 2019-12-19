use crate::config::KeywordsConfig;
use crate::context::Verbosity;

#[derive(Builder)]
#[builder(setter(into))]
pub struct EditSettings<'a> {
    pub keywords: &'a KeywordsConfig,
    #[builder(default)]
    pub verbosity: Verbosity,
}

impl<'a> EditSettings<'a> {
    pub fn builder() -> EditSettingsBuilder<'a> {
        EditSettingsBuilder::default()
    }
}
