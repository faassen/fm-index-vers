use crate::iter::{BackwardIterableIndex, BackwardIterator, ForwardIterableIndex, ForwardIterator};
use crate::suffix_array::IndexWithSA;

pub trait BackwardSearchIndex: BackwardIterableIndex {
    fn search_backward<K>(&self, pattern: K) -> Search<Self>
    where
        K: AsRef<[Self::T]>,
    {
        Search::new(self).search_backward(pattern)
    }
}

impl<I: BackwardIterableIndex> BackwardSearchIndex for I {}

pub struct Search<'a, I>
where
    I: BackwardSearchIndex,
{
    index: &'a I,
    s: u64,
    e: u64,
    pattern: Vec<I::T>,
}

impl<'a, I> Search<'a, I>
where
    I: BackwardSearchIndex,
{
    fn new(index: &'a I) -> Search<'a, I> {
        Search {
            index,
            s: 0,
            e: index.len(),
            pattern: vec![],
        }
    }

    pub fn search_backward<K: AsRef<[I::T]>>(&self, pattern: K) -> Self {
        let mut s = self.s;
        let mut e = self.e;
        let mut pattern = pattern.as_ref().to_vec();
        for &c in pattern.iter().rev() {
            s = self.index.lf_map2(c, s);
            e = self.index.lf_map2(c, e);
            if s == e {
                break;
            }
        }
        pattern.extend_from_slice(&self.pattern);

        Search {
            index: self.index,
            s,
            e,
            pattern,
        }
    }

    pub fn get_range(&self) -> (u64, u64) {
        (self.s, self.e)
    }

    pub fn count(&self) -> u64 {
        self.e - self.s
    }
}

impl<I> Search<'_, I>
where
    I: BackwardIterableIndex,
{
    pub fn iter_backward(&self, i: u64) -> BackwardIterator<I> {
        let m = self.count();

        debug_assert!(m > 0, "cannot iterate from empty search result");
        debug_assert!(i < m, "{} is out of range", i);

        self.index.iter_backward(self.s + i)
    }
}

impl<I> Search<'_, I>
where
    I: BackwardSearchIndex + ForwardIterableIndex,
{
    pub fn iter_forward(&self, i: u64) -> ForwardIterator<I> {
        let m = self.count();

        debug_assert!(m > 0, "cannot iterate from empty search result");
        debug_assert!(i < m, "{} is out of range", i);

        self.index.iter_forward(self.s + i)
    }
}

impl<I> Search<'_, I>
where
    I: BackwardSearchIndex + IndexWithSA,
{
    pub fn locate(&self) -> Vec<u64> {
        let mut results: Vec<u64> = Vec::with_capacity((self.e - self.s) as usize);
        for k in self.s..self.e {
            results.push(self.index.get_sa(k));
        }
        results
    }
}
