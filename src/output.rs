mod concise;
mod markdownlint;
mod mdl;

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    Concise,
    Mdl,
    Markdownlint,
}

pub use concise::Concise;
pub use markdownlint::Markdownlint;
pub use mdl::Mdl;
