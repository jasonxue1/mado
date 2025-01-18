use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD036 {
    pub punctuation: String,
}

impl Default for MD036 {
    #[inline]
    fn default() -> Self {
        Self {
            #[allow(clippy::use_self)]
            punctuation: rule::MD036::DEFAULT_PUNCTUATION.to_owned(),
        }
    }
}

#[allow(clippy::use_self)]
impl From<&MD036> for rule::MD036 {
    #[inline]
    fn from(config: &MD036) -> rule::MD036 {
        rule::MD036::new(config.punctuation.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md036() {
        let punctuation = "!?".to_owned();
        let config = MD036 {
            punctuation: punctuation.clone(),
        };
        let expected = rule::MD036::new(punctuation);
        assert_eq!(rule::MD036::from(&config), expected);
    }
}
