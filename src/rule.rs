use std::path::PathBuf;

use comrak::nodes::Sourcepos;
use miette::Result;

use crate::{violation::Violation, Document};

mod md001;
mod md002;
mod md003;
mod md004;
mod md005;
mod md006;
mod md007;
mod md009;
mod md010;
mod md012;
mod md013;
mod md014;
mod md018;
mod md019;
mod md022;

pub trait Rule: Send {
    fn name(&self) -> String;

    fn description(&self) -> String;

    fn tags(&self) -> Vec<String>;

    fn aliases(&self) -> Vec<String>;

    fn check(&self, doc: &Document) -> Result<Vec<Violation>>;

    #[inline]
    fn to_violation(&self, path: PathBuf, position: Sourcepos) -> Violation {
        Violation::new(path, self.name(), self.description(), position)
    }
}

pub use md001::MD001;
pub use md002::MD002;
pub use md003::MD003;
pub use md004::MD004;
pub use md005::MD005;
pub use md006::MD006;
pub use md007::MD007;
pub use md009::MD009;
pub use md010::MD010;
pub use md012::MD012;
pub use md013::MD013;
pub use md014::MD014;
pub use md018::MD018;
pub use md019::MD019;
pub use md022::MD022;
