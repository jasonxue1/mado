use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct MD002 {
    pub level: u8,
}

impl Default for MD002 {
    fn default() -> Self {
        Self { level: 1 }
    }
}
