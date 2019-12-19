use regex::Regex;

#[derive(Deserialize, Clone, Debug)]
pub struct KeywordsConfig {
    #[serde(with = "serde_regex")]
    single: Regex,
    #[serde(with = "serde_regex")]
    block_start: Regex,
    #[serde(with = "serde_regex")]
    block_end: Regex,
}
