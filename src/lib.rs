use std::ops::Range;

use tinyvec::TinyVec;

const INLINE_CAPACITY: usize = 2;

#[derive(Debug, Default)]
pub struct RangeSet(TinyVec<[Range<u64>; INLINE_CAPACITY]>);

impl Clone for RangeSet {
    fn clone(&self) -> Self {
        if self.0.is_inline() || self.0.len() > INLINE_CAPACITY {
            return Self(self.0.clone());
        }
        let mut vec = TinyVec::new();
        vec.extend_from_slice(self.0.as_slice());
        Self(vec)
    }
}

impl RangeSet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = Range<u64>> + '_ {
        self.0.iter().cloned()
    }

    /// Check if the range set contains a certain number
    pub fn contains(&self, num: u64) -> bool {
        for el in self.0.iter() {
            if el.start > num {
                return false;
            } else if el.contains(&num) {
                return true;
            }
        }
        false
    }

    /// Check if the range set is empty
    pub fn empty(&self) -> bool {
        return self.0.is_empty();
    }

    /// Uses `shrink_to_fit` of underlying tiny vec
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }

    /// Insert a Range into the range set
    /// uses std::ops::Range
    /// 
    /// example:
    /// 
    /// use std::ops::Range;
    /// 
    /// let range = std::ops::RangeSet{start: 1, end: 5};
    /// let mut set = RangeSet::new();
    /// set.insert(range);
    pub fn insert(&mut self, range: Range<u64>) {
        if range.is_empty() {
            return;
        }
        let mut index = 0;
        while index != self.0.len() {
            let current = &mut self.0[index];
            if current.start > range.end {
                self.0.insert(index, range);
                return;
            } else if current.start > range.start {
                current.start = range.start;
                return;
            }
            if range.end <= current.end {
                return;
            } else if range.start <= current.end {
                current.end = range.end;
                while index != self.0.len() - 1 {
                    let curr = self.0[index].clone();
                    let next = self.0[index + 1].clone();
                    if curr.end >= next.start {
                        self.0[index].end = next.end.max(curr.end);
                        self.0.remove(index + 1);
                    } else {
                        break;
                    }
                }
                return;
            }
            index += 1;
        }
        self.0.push(range);
        return;
    }

    /// Inserts range into the range set using a start and end number
    /// 
    /// example:
    /// 
    /// let mut set = RangeSet::new();
    /// let start = 1;
    /// let end = 5;
    /// set.insert_num(1, 5);
    pub fn insert_num(&mut self, start: u64, end: u64) {
        let range = std::ops::Range {
            start: start,
            end: end,
        };
        if range.is_empty() {
            return;
        }
        let mut index = 0;
        while index != self.0.len() {
            let current = &mut self.0[index];
            if current.start > range.end {
                self.0.insert(index, range);
                return;
            } else if current.start > range.start {
                current.start = range.start;
                return;
            }
            if range.end <= current.end {
                return;
            } else if range.start <= current.end {
                current.end = range.end;
                while index != self.0.len() - 1 {
                    let curr = self.0[index].clone();
                    let next = self.0[index + 1].clone();
                    if curr.end >= next.start {
                        self.0[index].end = next.end.max(curr.end);
                        self.0.remove(index + 1);
                    } else {
                        break;
                    }
                }
                return;
            }
            index += 1;
        }
        self.0.push(range);
        return;
    }

    /// Removes a range from the range set
    /// uses use std::ops::Range
    pub fn remove(&mut self, range: Range<u64>) {
        if range.is_empty() {
            return;
        }
        let mut index = 0;
        while index != self.0.len() && range.start != range.end {
            let current = self.0[index].clone();

            if range.end <= current.start {
                return;
            } else if range.start >= current.end {
                index += 1;
                continue;
            }
            let left = current.start..range.start;
            let right = range.end..current.end;
            if left.is_empty() && right.is_empty() {
                self.0.remove(index);
            } else if left.is_empty() {
                self.0[index] = right;
                index += 1;
            } else if right.is_empty() {
                self.0[index] = left;
                index += 1;
            } else {
                self.0[index] = right;
                self.0.insert(index, left);
                index += 2;
            }
        }
        return;
    }
}
