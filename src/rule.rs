use markdown::unist::Position;
use miette::Result;

use crate::violation::Violation;

mod md001;
mod md002;
mod md005;
mod md022;

pub trait Rule {
    fn name(&self) -> String;

    fn description(&self) -> String;

    fn tags(&self) -> Vec<String>;

    fn aliases(&self) -> Vec<String>;

    fn check(&self, doc: &markdown::mdast::Node) -> Result<Vec<Violation>>;

    #[inline]
    fn to_violation(&self, position: Position) -> Violation {
        Violation::new(self.name(), self.description(), position)
    }
}

pub use md001::MD001;
pub use md002::MD002;
pub use md005::MD005;
pub use md022::MD022;
