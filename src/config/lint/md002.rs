use serde::{Deserialize, Serialize};

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD002 {
    pub level: u8,
}

impl Default for MD002 {
    #[inline]
    fn default() -> Self {
        Self {
            level: rule::MD002::DEFAULT_LEVEL,
        }
    }
}

impl From<&MD002> for rule::MD002 {
    #[inline]
    fn from(config: &MD002) -> Self {
        Self::new(config.level)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md002() {
        let level = 3;
        let config = MD002 { level };
        let expected = rule::MD002::new(level);
        assert_eq!(rule::MD002::from(&config), expected);
    }
}
