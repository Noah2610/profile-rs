use regex::Regex;

#[derive(Deserialize, Debug)]
pub struct ConfigKeywords {
    #[serde(with = "serde_regex")]
    single: Regex,
    #[serde(with = "serde_regex")]
    block_start: Regex,
    #[serde(with = "serde_regex")]
    block_end: Regex,
}
