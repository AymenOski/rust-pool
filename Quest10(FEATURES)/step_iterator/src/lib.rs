pub struct StepIterator<T> {
    pub beg: T,
    pub end: T,
    pub step: T,
}

use std::ops::Add;
impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self { beg, end, step }
    }
}

impl<T: Add<Output = T> + PartialOrd + Copy> std::iter::Iterator for StepIterator<T> {
    type Item = T;  // Define the type of items the iterator will produce
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.beg;
        if self.beg + self.step <= self.end {
            self.beg += self.step;
        } else {
            return None;
        }
        Some(current)
    }
}
