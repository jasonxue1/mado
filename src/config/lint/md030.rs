use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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
