use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum Tag {
    Atx,
    AtxClosed,
    BlankLines,
    Blockquote,
    Bullet,
    Code,
    Emphasis,
    HardTab,
    Headers,
    Hr,
    Html,
    Indentation,
    Language,
    LineLength,
    Links,
    Ol,
    Spaces,
    Ul,
    Url,
    Whitespace,
}
