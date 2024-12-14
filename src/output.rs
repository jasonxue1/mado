mod concise;
mod mdl;

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    Concise,
    Mdl,
}

pub use concise::Concise;
pub use mdl::Mdl;
