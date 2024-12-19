use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct MD036 {
    pub punctuation: String,
}

impl Default for MD036 {
    fn default() -> Self {
        Self {
            punctuation: ".,;:!?".to_owned(),
        }
    }
}
