struct NumArray {
    inner: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self { inner: nums }
    }

    fn update(&mut self, i: i32, val: i32) {
        self.inner[i as usize] = val;
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let (i, j) = (i as usize, j as usize);
        self.inner[i..=j].iter().sum()
    }
}
