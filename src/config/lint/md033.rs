use serde::{Deserialize, Serialize};

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
#[allow(clippy::exhaustive_structs)]
pub struct MD033 {
    pub allowed_elements: Vec<String>,
}

impl Default for MD033 {
    #[inline]
    fn default() -> Self {
        Self {
            allowed_elements: rule::MD033::DEFAULT_ALLOWED_ELEMENTS,
        }
    }
}

impl From<&MD033> for rule::MD033 {
    #[inline]
    fn from(config: &MD033) -> Self {
        Self::new(&config.allowed_elements)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md033() {
        let allowed_elements = vec!["br".to_owned()];
        let config = MD033 {
            allowed_elements: allowed_elements.clone(),
        };
        let expected = rule::MD033::new(&allowed_elements);
        assert_eq!(rule::MD033::from(&config), expected);
    }
}
