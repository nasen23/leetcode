const BLOCK_SIZE: usize = 100;

struct NumArray {
    nums: Vec<i32>,
    block_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut i = 0;
        let mut block_sum = vec![];
        while i + BLOCK_SIZE <= nums.len() {
            block_sum.push(nums.iter().skip(i).take(BLOCK_SIZE).sum());
            i += BLOCK_SIZE;
        }
        Self { nums, block_sum }
    }

    fn update(&mut self, i: i32, val: i32) {
        let i = i as usize;
        if i / BLOCK_SIZE < self.block_sum.len() {
            let delta = val - self.nums[i];
            self.block_sum[i / BLOCK_SIZE] += delta;
        }
        self.nums[i] = val;
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let (i, j) = (i as usize, j as usize);
        let mut k = i;
        let mut res = 0;
        while k <= j {
            if k % BLOCK_SIZE == 0 && k + BLOCK_SIZE - 1 <= j {
                res += self.block_sum[k / BLOCK_SIZE];
                k += BLOCK_SIZE;
            } else {
                res += self.nums[k];
                k += 1;
            }
        }
        res
    }
}
