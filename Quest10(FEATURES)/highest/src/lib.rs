#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.iter().last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        let mut arr: Vec<u32> = self.numbers.iter().copied().collect();
        arr.sort();
        arr.iter().last().copied()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut arr: Vec<u32> = self.numbers.iter().copied().collect();
        arr.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let res = arr.iter().take(3).copied().collect();
        res
    }
}
