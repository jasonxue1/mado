use std::path::PathBuf;

use comrak::nodes::Sourcepos;
use miette::Result;
use serde::Deserialize;

use crate::{violation::Violation, Document};

mod helper;
mod md001;
mod md002;
pub mod md003;
pub mod md004;
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
mod md023;
mod md024;
mod md025;
mod md026;
mod md027;
mod md028;
pub mod md029;
mod md030;
mod md031;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub enum Rule {
    MD001,
    MD002,
    MD003,
    MD004,
    MD005,
    MD006,
    MD007,
    MD009,
    MD010,
    MD012,
    MD013,
    MD014,
    MD018,
    MD019,
    MD022,
    MD023,
    MD024,
    MD025,
    MD026,
    MD027,
    MD028,
    MD029,
    MD030,
    MD031,
}

pub trait RuleLike: Send {
    fn name(&self) -> String;

    fn description(&self) -> String;

    fn tags(&self) -> Vec<String>;

    fn aliases(&self) -> Vec<String>;

    fn check(&self, doc: &Document) -> Result<Vec<Violation>>;

    #[inline]
    fn to_violation(&self, path: PathBuf, position: Sourcepos) -> Violation {
        Violation::new(
            path,
            self.name(),
            self.aliases()[0].clone(),
            self.description(),
            position,
        )
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
pub use md023::MD023;
pub use md024::MD024;
pub use md025::MD025;
pub use md026::MD026;
pub use md027::MD027;
pub use md028::MD028;
pub use md029::MD029;
pub use md030::MD030;
pub use md031::MD031;
