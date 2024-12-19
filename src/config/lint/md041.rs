use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct MD041 {
    pub level: u8,
}

impl Default for MD041 {
    fn default() -> Self {
        Self { level: 1 }
    }
}
