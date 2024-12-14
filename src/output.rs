mod concise;
mod mdl;

#[allow(unused)]
pub enum Format {
    Concise,
    Mdl,
}

// TODO
#[allow(unused_imports)]
pub use concise::Concise;
#[allow(unused_imports)]
pub use mdl::Mdl;
