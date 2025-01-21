use serde::{Deserialize, Serialize};

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD030 {
    pub ul_single: usize,
    pub ol_single: usize,
    pub ul_multi: usize,
    pub ol_multi: usize,
}

impl Default for MD030 {
    #[inline]
    fn default() -> Self {
        Self {
            ul_single: rule::MD030::DEFAULT_UL_SINGLE,
            ol_single: rule::MD030::DEFAULT_OL_SINGLE,
            ul_multi: rule::MD030::DEFAULT_UL_MULTI,
            ol_multi: rule::MD030::DEFAULT_OL_MULTI,
        }
    }
}

impl From<&MD030> for rule::MD030 {
    #[inline]
    fn from(config: &MD030) -> Self {
        Self::new(
            config.ul_single,
            config.ol_single,
            config.ul_multi,
            config.ol_multi,
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md030() {
        let ul_single = 1;
        let ol_single = 2;
        let ul_multi = 3;
        let ol_multi = 4;
        let config = MD030 {
            ul_single,
            ol_single,
            ul_multi,
            ol_multi,
        };
        let expected = rule::MD030::new(ul_single, ol_single, ul_multi, ol_multi);
        assert_eq!(rule::MD030::from(&config), expected);
    }
}
