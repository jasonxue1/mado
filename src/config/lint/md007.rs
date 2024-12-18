use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct MD007 {
    pub indent: usize,
}

impl Default for MD007 {
    fn default() -> Self {
        Self { indent: 4 }
    }
}
