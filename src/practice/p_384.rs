use rand::prelude::*;

struct Solution {
    original_vec: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { original_vec: nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.original_vec.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut ans = self.original_vec.clone();
        let mut rng = thread_rng();
        for n in (1..=ans.len()).rev() {
            let idx = rng.gen_range(0..n);
            let last_idx = n - 1;
            // swap
            (ans[idx], ans[last_idx]) = (ans[last_idx], ans[idx])
        }

        ans
    }
}

// /**
//  * Your Solution object will be instantiated and called as such:
//  * let obj = Solution::new(nums);
//  * let ret_1: Vec<i32> = obj.reset();
//  * let ret_2: Vec<i32> = obj.shuffle();
//  */
