use crate::config::KeywordsConfig;

#[derive(Builder)]
#[builder(setter(into))]
pub struct EditSettings<'a> {
    pub keywords: &'a KeywordsConfig,
}

impl<'a> EditSettings<'a> {
    pub fn builder() -> EditSettingsBuilder<'a> {
        EditSettingsBuilder::default()
    }
}
