use clap::Parser;
use markdown::mdast::Node;

use markdownlint::rule;
use markdownlint::Cli;
use markdownlint::Command;
use markdownlint::Rule;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // TODO
        Some(Command::Check { files }) => {
            let text = "# Hello
        ### Cluel
        ## World
        ";
            let doc: Node = markdown::to_mdast(text, &markdown::ParseOptions::default()).unwrap();

            let md001 = rule::MD001::new();

            println!("{:?}", md001.check(&doc));
            println!("{:?}", doc);
            println!("{:?}", files);
        }
        None => {}
    }
}
