use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD002 {
    pub level: u8,
}

impl Default for MD002 {
    #[inline]
    fn default() -> Self {
        Self {
            #[allow(clippy::use_self)]
            level: rule::MD002::DEFAULT_LEVEL,
        }
    }
}

#[allow(clippy::use_self)]
impl From<&MD002> for rule::MD002 {
    #[inline]
    fn from(config: &MD002) -> rule::MD002 {
        rule::MD002::new(config.level)
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
