use std::fs;
use std::path::Path;

use etcetera::choose_base_strategy;
use etcetera::BaseStrategy as _;
use miette::miette;
use miette::IntoDiagnostic as _;
use miette::Result;
use serde::Deserialize;

mod lint;

pub use lint::Lint;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub lint: Lint,
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Config> {
    let config_text = fs::read_to_string(path).into_diagnostic()?;
    toml::from_str(&config_text).map_err(|err| miette!(err))
}

const FILE_NAME: &str = "downlint.toml";
const HIDDEN_FILE_NAME: &str = ".downlint.toml";

pub fn resolve() -> Result<Config> {
    let local_path = Path::new(FILE_NAME);
    let exists_local = fs::exists(local_path).into_diagnostic()?;
    if exists_local {
        return load(local_path);
    }

    let hidden_local_path = Path::new(HIDDEN_FILE_NAME);
    let exists_hidden_local = fs::exists(hidden_local_path).into_diagnostic()?;
    if exists_hidden_local {
        return load(hidden_local_path);
    }

    let strategy = choose_base_strategy().into_diagnostic()?;
    let mut config_path = strategy.config_dir();
    config_path.push(FILE_NAME);
    let exists_config = fs::exists(&config_path).into_diagnostic()?;
    if exists_config {
        return load(&config_path);
    }

    Ok(Config::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{output::Format, Rule};
    use lint::MD002;

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
        expected.lint.rules = vec![Rule::MD027];
        expected.lint.md002 = MD002 { level: 2 };
        assert_eq!(actual, expected);
    }
}
