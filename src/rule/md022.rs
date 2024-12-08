use markdown::mdast::Node;

use crate::violation::Violation;

use super::Rule;

#[derive(Default)]
pub struct MD022 {}

impl MD022 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD022 {
    fn name(&self) -> String {
        "MD022".to_string()
    }

    fn description(&self) -> String {
        "Headers should be surrounded by blank lines".to_string()
    }

    fn tags(&self) -> Vec<String> {
        vec!["headers".to_string(), "blank_lines".to_string()]
    }

    fn aliases(&self) -> Vec<String> {
        vec!["blanks-around-headers".to_string()]
    }

    fn check(&self, doc: &Node) -> Vec<Violation> {
        match doc.children() {
            Some(children) => {
                children
                    .iter()
                    .fold(
                        (vec![], None::<&Node>),
                        |(acc, maybe_prev), node| match maybe_prev {
                            Some(prev) => {
                                // TODO: Don't use unwrap
                                let mut vec = acc.clone();

                                if let Node::Heading(_) = node {
                                    let prev_position = prev.position().unwrap();
                                    let position = node.position().unwrap();
                                    if position.start.line == prev_position.end.line + 1 {
                                        let violation = Violation::new(
                                            self.name(),
                                            self.description(),
                                            position.clone(),
                                        );
                                        vec.push(violation);
                                    }
                                } else if let Node::Heading(_) = prev {
                                    let prev_position = prev.position().unwrap();
                                    let position = node.position().unwrap();
                                    if position.start.line == prev_position.end.line + 1 {
                                        let violation = Violation::new(
                                            self.name(),
                                            self.description(),
                                            prev_position.clone(),
                                        );
                                        vec.push(violation);
                                    }
                                }

                                (vec, Some(node))
                            }
                            None => (acc, Some(node)),
                        },
                    )
                    .0
            }
            None => vec![],
        }
    }
}
