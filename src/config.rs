use std::fs;
use std::path::Path;

use etcetera::choose_base_strategy;
use etcetera::BaseStrategy as _;
use miette::miette;
use miette::IntoDiagnostic as _;
use miette::Result;
use serde::Deserialize;

pub mod lint;

pub use lint::Lint;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct Config {
    pub lint: Lint,
}

impl Config {
    const FILE_NAME: &str = "mado.toml";
    const HIDDEN_FILE_NAME: &str = ".mado.toml";

    #[inline]
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config_text = fs::read_to_string(path).into_diagnostic()?;
        toml::from_str(&config_text).map_err(|err| miette!(err))
    }

    #[inline]
    pub fn resolve() -> Result<Self> {
        let local_path = Path::new(Self::FILE_NAME);
        let exists_local = fs::exists(local_path).into_diagnostic()?;
        if exists_local {
            return Self::load(local_path);
        }

        let hidden_local_path = Path::new(Self::HIDDEN_FILE_NAME);
        let exists_hidden_local = fs::exists(hidden_local_path).into_diagnostic()?;
        if exists_hidden_local {
            return Self::load(hidden_local_path);
        }

        let strategy = choose_base_strategy().into_diagnostic()?;
        let config_path = strategy.config_dir().join("mado").join(Self::FILE_NAME);
        let exists_config = fs::exists(&config_path).into_diagnostic()?;
        if exists_config {
            return Self::load(&config_path);
        }

        Ok(Self::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::output::Format;
    use lint::{RuleSet, MD002};

    #[test]
    fn load() {
        let path = Path::new("mado.toml");
        let actual = Config::load(path).unwrap();
        let mut expected = Config::default();
        expected.lint.rules.remove(18); // Remove MD024
        expected.lint.md013.code_blocks = false;
        expected.lint.md013.tables = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn resolve() {
        let actual = Config::resolve().unwrap();
        let path = Path::new("mado.toml");
        let expected = Config::load(path).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn deserialize() {
        let text = r#"[lint]
output-format = "mdl"
rules = ["MD027"]

[lint.md002]
level = 2
"#;
        let actual: Config = toml::from_str(text).unwrap();
        let mut expected = Config::default();
        expected.lint.output_format = Format::Mdl;
        expected.lint.rules = vec![RuleSet::MD027];
        expected.lint.md002 = MD002 { level: 2 };
        assert_eq!(actual, expected);
    }
}
