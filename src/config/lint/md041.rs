use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD041 {
    pub level: u8,
}

impl Default for MD041 {
    #[inline]
    fn default() -> Self {
        Self {
            level: rule::MD041::DEFAULT_LEVEL,
        }
    }
}

impl From<&MD041> for rule::MD041 {
    #[inline]
    fn from(config: &MD041) -> Self {
        Self::new(config.level)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md041() {
        let level = 3;
        let config = MD041 { level };
        let expected = rule::MD041::new(level);
        assert_eq!(rule::MD041::from(&config), expected);
    }
}
