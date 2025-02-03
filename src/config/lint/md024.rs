use serde::{Deserialize, Serialize};

use crate::rule;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
#[allow(clippy::exhaustive_structs)]
pub struct MD024 {
    pub allow_different_nesting: bool,
}

impl From<&MD024> for rule::MD024 {
    #[inline]
    fn from(config: &MD024) -> Self {
        Self::new(config.allow_different_nesting)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md024() {
        let allow_different_nesting = true;
        let config = MD024 {
            allow_different_nesting,
        };
        let expected = rule::MD024::new(allow_different_nesting);
        assert_eq!(rule::MD024::from(&config), expected);
    }
}
