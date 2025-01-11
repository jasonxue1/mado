use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD007 {
    pub indent: usize,
}

impl Default for MD007 {
    #[inline]
    fn default() -> Self {
        Self {
            indent: rule::MD007::DEFAULT_INDENT,
        }
    }
}

impl From<&MD007> for rule::MD007 {
    #[inline]
    fn from(config: &MD007) -> rule::MD007 {
        rule::MD007::new(config.indent)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md007() {
        let indent = 5;
        let config = MD007 { indent };
        let expected = rule::MD007::new(indent);
        assert_eq!(rule::MD007::from(&config), expected);
    }
}
