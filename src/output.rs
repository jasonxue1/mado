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
        // TODO: Reduce clone
        match self {
            Self::Concise => |a, b| Concise::new(a.clone()).cmp(&Concise::new(b.clone())),
            Self::Mdl => |a, b| Mdl::new(a.clone()).cmp(&Mdl::new(b.clone())),
            Self::Markdownlint => {
                |a, b| Markdownlint::new(a.clone()).cmp(&Markdownlint::new(b.clone()))
            }
        }
    }
}

pub use concise::Concise;
pub use markdownlint::Markdownlint;
pub use mdl::Mdl;

use crate::Violation;
