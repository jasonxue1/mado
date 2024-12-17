use core::hash::Hash;
use core::marker::PhantomData;
use core::ops::RangeBounds;
use std::collections::hash_set::Iter;
use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct RangeSet<R: RangeBounds<Idx>, Idx> {
    data: HashSet<R>,
    phantom: PhantomData<Idx>,
}

impl<R, Idx> RangeSet<R, Idx>
where
    R: RangeBounds<Idx>,
{
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        let data = HashSet::new();
        let phantom = PhantomData;
        Self { data, phantom }
    }

    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_, R> {
        self.data.iter()
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl<R, Idx> RangeSet<R, Idx>
where
    R: RangeBounds<Idx>,
    Idx: PartialOrd,
{
    #[inline]
    #[must_use]
    pub fn contains(&self, value: &Idx) -> bool {
        self.data.iter().any(|range| range.contains(value))
    }
}

impl<R, Idx> RangeSet<R, Idx>
where
    R: RangeBounds<Idx> + Eq + Hash,
{
    #[inline]
    pub fn insert(&mut self, value: R) {
        self.data.insert(value);
    }
}

impl<R, Idx> PartialEq for RangeSet<R, Idx>
where
    R: RangeBounds<Idx> + Eq + Hash,
{
    #[inline]
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<R, Idx> IntoIterator for RangeSet<R, Idx>
where
    R: RangeBounds<Idx>,
{
    type Item = R;
    type IntoIter = <HashSet<R, Idx> as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, R, Idx> IntoIterator for &'a RangeSet<R, Idx>
where
    R: RangeBounds<Idx>,
{
    type Item = &'a R;
    type IntoIter = Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn insert() {
        let mut set = RangeSet::new();
        assert!(!set.data.contains(&(0..10)));
        set.insert(0..10);
        assert!(set.data.contains(&(0..10)));
    }

    #[test]
    fn len() {
        let mut set = RangeSet::new();
        assert_eq!(set.len(), 0);
        set.insert(0..10);
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn is_empty() {
        let mut set = RangeSet::new();
        assert!(set.is_empty());
        set.insert(0..10);
        assert!(!set.is_empty());
    }

    #[test]
    fn contains_range() {
        let mut set = RangeSet::new();
        set.insert(0..10);
        set.insert(20..30);
        set.insert(25..35);
        assert!(set.contains(&0));
        assert!(!set.contains(&10));
        assert!(set.contains(&20));
        assert!(set.contains(&30));
        assert!(!set.contains(&35));
    }

    #[test]
    fn contains_range_inclusive() {
        let mut set = RangeSet::new();
        set.insert(0..=10);
        set.insert(20..=30);
        set.insert(25..=35);
        assert!(set.contains(&0));
        assert!(set.contains(&10));
        assert!(set.contains(&20));
        assert!(set.contains(&30));
        assert!(set.contains(&35));
    }
}
