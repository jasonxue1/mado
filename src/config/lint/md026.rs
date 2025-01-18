use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
#[allow(clippy::exhaustive_structs)]
pub struct MD026 {
    pub punctuation: String,
}

impl Default for MD026 {
    #[inline]
    fn default() -> Self {
        Self {
            #[allow(clippy::use_self)]
            punctuation: rule::MD026::DEFAULT_PUNCTUATION.to_owned(),
        }
    }
}

#[allow(clippy::use_self)]
impl From<&MD026> for rule::MD026 {
    #[inline]
    fn from(config: &MD026) -> rule::MD026 {
        rule::MD026::new(config.punctuation.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md026() {
        let punctuation = "!?".to_owned();
        let config = MD026 {
            punctuation: punctuation.clone(),
        };
        let expected = rule::MD026::new(punctuation);
        assert_eq!(rule::MD026::from(&config), expected);
    }
}
