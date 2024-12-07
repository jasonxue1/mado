use std::ffi::OsStr;
use std::fs;

use clap::Parser;

use markdownlint::rule;
use markdownlint::Cli;
use markdownlint::Command;
use markdownlint::Rule;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Check { files }) => {
            let md001 = rule::MD001::new();

            for file in files {
                let path = file.as_path();
                if path.is_dir() {
                    // TODO
                } else if path.is_file() && path.extension() == Some(OsStr::new("md")) {
                    let text = &fs::read_to_string(path).unwrap(); // TODO: Don't use unwrap
                    let doc = markdown::to_mdast(text, &markdown::ParseOptions::default()).unwrap();
                    println!("{:?}", md001.check(&doc));
                } else {
                    // TODO
                }
            }
        }
        None => {}
    }
}
