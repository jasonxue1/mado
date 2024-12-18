use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct MD030 {
    pub ul_single: usize,
    pub ol_single: usize,
    pub ul_multi: usize,
    pub ol_multi: usize,
}

impl Default for MD030 {
    fn default() -> Self {
        Self {
            ul_single: 1,
            ol_single: 1,
            ul_multi: 1,
            ol_multi: 1,
        }
    }
}
