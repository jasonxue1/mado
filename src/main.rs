use pulldown_cmark::{html::push_html, Parser};

use markdownlint::Rule;

fn main() {
    let parser = Parser::new("hello world");

    let mut html_output = String::new();
    push_html(&mut html_output, parser);

    Rule {
        name: "MD001".to_string(),
        description: "Header levels should only increment by one level at a time".to_string(),
        tags: vec!["headers".to_string()],
        aliases: vec!["header-increment".to_string()],
        check: { |_doc| vec![0] },
    };

    println!("{html_output}");
}
