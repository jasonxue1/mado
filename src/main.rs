use markdownlint::Rule;

fn main() {
    Rule {
        name: "MD001".to_string(),
        description: "Header levels should only increment by one level at a time".to_string(),
        tags: vec!["headers".to_string()],
        aliases: vec!["header-increment".to_string()],
        check: { || vec![0] },
    };

    println!("Hello, world!");
}
