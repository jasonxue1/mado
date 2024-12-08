use markdown::unist::Position;
use miette::Result;

use crate::{violation::Violation, Document};

mod md001;
mod md002;
mod md005;
mod md009;
mod md010;
mod md012;
mod md022;

pub trait Rule {
    fn name(&self) -> String;

    fn description(&self) -> String;

    fn tags(&self) -> Vec<String>;

    fn aliases(&self) -> Vec<String>;

    fn check(&self, doc: &Document) -> Result<Vec<Violation>>;

    #[inline]
    fn to_violation(&self, position: Position) -> Violation {
        Violation::new(self.name(), self.description(), position)
    }
}

pub use md001::MD001;
pub use md002::MD002;
pub use md005::MD005;
pub use md009::MD009;
pub use md010::MD010;
pub use md012::MD012;
pub use md022::MD022;
