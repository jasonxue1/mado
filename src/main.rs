use markdown::mdast::Node;

use markdownlint::Rule;

fn main() {
    let text = "# Hello
### Cluel
## World
";
    let doc: Node = markdown::to_mdast(text, &markdown::ParseOptions::default()).unwrap();

    let md001 = Rule {
        name: "MD001".to_string(),
        description: "Header levels should only increment by one level at a time".to_string(),
        tags: vec!["headers".to_string()],
        aliases: vec!["header-increment".to_string()],
        check: {
            |root| match root.children() {
                Some(children) => {
                    children
                        .iter()
                        .fold((vec![], 0), |(acc, old_level), node| match node {
                            Node::Heading(heading) => {
                                let mut vec = acc.clone();
                                if heading.depth < old_level {
                                    // TODO: Don't use unwrap
                                    vec.push(heading.position.clone().unwrap());
                                }
                                (vec, heading.depth)
                            }
                            _ => (acc, old_level),
                        })
                        .0
                }
                None => vec![],
            }
        },
    };

    println!("{:?}", (md001.check)(&doc));
    println!("{:?}", doc);
}
