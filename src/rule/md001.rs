use markdown::mdast::Node;

use super::Rule;

pub struct MD001 {}

impl MD001 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD001 {
    fn name(&self) -> String {
        "MD001".to_string()
    }

    fn description(&self) -> String {
        "Header levels should only increment by one level at a time".to_string()
    }

    fn tags(&self) -> Vec<String> {
        vec!["headers".to_string()]
    }

    fn aliases(&self) -> Vec<String> {
        vec!["header-increment".to_string()]
    }

    fn check(&self, doc: &markdown::mdast::Node) -> Vec<markdown::unist::Position> {
        match doc.children() {
            Some(children) => {
                children
                    .iter()
                    .fold((vec![], 0), |(acc, old_depth), node| match node {
                        Node::Heading(heading) => {
                            let mut vec = acc.clone();
                            if heading.depth != old_depth + 1 {
                                // TODO: Don't use unwrap
                                vec.push(heading.position.clone().unwrap());
                            }
                            (vec, heading.depth)
                        }
                        _ => (acc, old_depth),
                    })
                    .0
            }
            None => vec![],
        }
    }
}
