use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MD013 {
    pub line_length: usize,
}

impl Default for MD013 {
    fn default() -> Self {
        Self { line_length: 80 }
    }
}
