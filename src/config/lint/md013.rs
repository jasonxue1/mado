use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
#[allow(clippy::exhaustive_structs)]
pub struct MD013 {
    pub line_length: usize,
    pub code_blocks: bool,
    pub tables: bool,
}

impl Default for MD013 {
    #[inline]
    fn default() -> Self {
        Self {
            line_length: rule::MD013::DEFAULT_LINE_LENGTH,
            code_blocks: rule::MD013::DEFAULT_CODE_BLOCKS,
            tables: rule::MD013::DEFAULT_TABLES,
        }
    }
}
