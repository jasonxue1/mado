use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
#[allow(clippy::exhaustive_structs)]
pub struct MD025 {
    pub level: u8,
}

impl Default for MD025 {
    #[inline]
    fn default() -> Self {
        Self {
            #[allow(clippy::use_self)]
            level: rule::MD025::DEFAULT_LEVEL,
        }
    }
}

#[allow(clippy::use_self)]
impl From<&MD025> for rule::MD025 {
    #[inline]
    fn from(config: &MD025) -> rule::MD025 {
        rule::MD025::new(config.level)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md025() {
        let level = 3;
        let config = MD025 { level };
        let expected = rule::MD025::new(level);
        assert_eq!(rule::MD025::from(&config), expected);
    }
}
