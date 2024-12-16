#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::perf)]
#![warn(clippy::suspicious)]
// pedantic
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// restrictions
#![warn(clippy::absolute_paths)]
#![warn(clippy::blanket_clippy_restriction_lints)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::exhaustive_enums)]
#![warn(clippy::exhaustive_structs)]
#![warn(clippy::expect_used)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::missing_inline_in_public_items)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::std_instead_of_alloc)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::str_to_string)]
#![warn(clippy::unused_trait_names)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
// TODO: Enable following rules
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::integer_division_remainder_used)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::print_stdout)]

mod cli;
pub mod collection;
pub mod command;
mod document;
mod output;
pub mod rule;
pub mod service;
mod violation;

pub use cli::Cli;
pub use command::Command;
pub use document::Document;
pub use rule::Rule;
pub use violation::Violation;
