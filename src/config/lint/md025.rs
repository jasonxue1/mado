use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MD025 {
    pub level: u8,
}

impl Default for MD025 {
    fn default() -> Self {
        Self { level: 1 }
    }
}
