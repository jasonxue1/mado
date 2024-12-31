use core::cmp::Ordering;

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

impl Format {
    #[inline]
    #[must_use]
    pub fn sorter(&self) -> fn(a: &Violation, b: &Violation) -> Ordering {
        match self {
            Self::Concise => |a, b| Concise::new(a).cmp(&Concise::new(b)),
            Self::Mdl => |a, b| Mdl::new(a).cmp(&Mdl::new(b)),
            Self::Markdownlint => |a, b| Markdownlint::new(a).cmp(&Markdownlint::new(b)),
        }
    }
}

pub use concise::Concise;
pub use markdownlint::Markdownlint;
pub use mdl::Mdl;

use crate::Violation;
