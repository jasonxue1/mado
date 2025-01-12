#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Metadata {
    pub name: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub aliases: &'static [&'static str],
}
