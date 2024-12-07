mod md001;

pub trait Rule {
    fn name(&self) -> String;

    fn description(&self) -> String;

    fn tags(&self) -> Vec<String>;

    fn aliases(&self) -> Vec<String>;

    fn check(&self, doc: &markdown::mdast::Node) -> Vec<markdown::unist::Position>;
}

pub use md001::MD001;
