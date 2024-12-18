use clap::ValueEnum;
use serde::Deserialize;

mod concise;
mod markdownlint;
mod mdl;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Concise,
    Mdl,
    Markdownlint,
}

pub use concise::Concise;
pub use markdownlint::Markdownlint;
pub use mdl::Mdl;
