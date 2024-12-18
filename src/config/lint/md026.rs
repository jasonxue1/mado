use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MD026 {
    pub punctuation: String,
}

impl Default for MD026 {
    fn default() -> Self {
        Self {
            punctuation: ".,;:!?".to_owned(),
        }
    }
}
