use serde::Deserialize;

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MD033 {
    pub allowed_elements: Vec<String>,
}
