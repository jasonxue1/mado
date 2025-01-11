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

impl From<&MD013> for rule::MD013 {
    #[inline]
    fn from(config: &MD013) -> rule::MD013 {
        rule::MD013::new(config.line_length, config.code_blocks, config.tables)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md013() {
        let line_length = 33;
        let code_blocks = true;
        let tables = false;
        let config = MD013 {
            line_length,
            code_blocks,
            tables,
        };
        let expected = rule::MD013::new(line_length, code_blocks, tables);
        assert_eq!(rule::MD013::from(&config), expected);
    }
}
