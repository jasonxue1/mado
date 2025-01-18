use core::cmp::Ordering;
use std::path::PathBuf;

use comrak::nodes::Sourcepos;

use crate::rule::Metadata;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    path: PathBuf,
    metadata: &'static Metadata,
    position: Sourcepos,
}

impl Violation {
    #[inline]
    #[must_use]
    pub fn new(path: PathBuf, metadata: &'static Metadata, position: Sourcepos) -> Self {
        Self {
            path,
            metadata,
            position,
        }
    }

    #[inline]
    #[must_use]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        self.metadata.name
    }

    #[inline]
    #[must_use]
    pub fn alias(&self) -> &str {
        self.metadata.aliases[0]
    }

    #[inline]
    #[must_use]
    pub fn description(&self) -> &str {
        self.metadata.description
    }

    #[inline]
    #[must_use]
    pub fn position(&self) -> &Sourcepos {
        &self.position
    }
}

impl PartialOrd for Violation {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Violation {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let path_cmp = self.path.cmp(&other.path);
        if path_cmp != Ordering::Equal {
            return path_cmp;
        }

        let position_cmp = self.position.start.cmp(&other.position().start);
        if position_cmp != Ordering::Equal {
            return position_cmp;
        }

        self.metadata.name.cmp(other.metadata.name)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::nodes::Sourcepos;
    use pretty_assertions::assert_eq;

    use crate::rule::RuleLike as _;
    use crate::rule::{MD001, MD010};

    #[test]
    #[allow(clippy::similar_names)]
    fn cmp() {
        let path1 = Path::new("foo.md").to_path_buf();
        let path2 = Path::new("bar.md").to_path_buf();
        let md001 = MD001::new();
        let md010 = MD010::new();
        let position0 = Sourcepos::from((1, 1, 1, 1));
        let position1 = Sourcepos::from((1, 2, 1, 2));
        let position2 = Sourcepos::from((2, 1, 2, 1));
        let violation1 = md001.to_violation(path1.clone(), position0);
        let violation2 = md001.to_violation(path1.clone(), position1);
        let violation3 = md001.to_violation(path1.clone(), position2);
        let violation4 = md001.to_violation(path2.clone(), position0);
        let violation5 = md001.to_violation(path2.clone(), position1);
        let violation6 = md001.to_violation(path2.clone(), position2);
        let violation7 = md010.to_violation(path1.clone(), position0);
        let violation8 = md010.to_violation(path1.clone(), position1);
        let violation9 = md010.to_violation(path1, position2);
        let violation10 = md010.to_violation(path2.clone(), position0);
        let violation11 = md010.to_violation(path2.clone(), position1);
        let violation12 = md010.to_violation(path2, position2);
        let mut actual = vec![
            violation1.clone(),
            violation2.clone(),
            violation3.clone(),
            violation4.clone(),
            violation5.clone(),
            violation6.clone(),
            violation7.clone(),
            violation8.clone(),
            violation9.clone(),
            violation10.clone(),
            violation11.clone(),
            violation12.clone(),
        ];
        actual.sort();
        let expected = vec![
            violation4,
            violation10,
            violation5,
            violation11,
            violation6,
            violation12,
            violation1,
            violation7,
            violation2,
            violation8,
            violation3,
            violation9,
        ];
        assert_eq!(actual, expected);
    }
}
