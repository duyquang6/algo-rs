use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut ans = 0i64;
        let (mut left, mut right) = (0usize, 0usize);
        let n = nums.len();
        let mut pair_count = 0;
        let mut fq = HashMap::new();
        while left < n {
            while pair_count < k && right < n {
                // move right
                pair_count += fq.get(&nums[right]).unwrap_or(&0);
                dbg!(left, right, pair_count);
                fq.insert(nums[right], *fq.get(&nums[right]).unwrap_or(&0) + 1);
                right += 1;
            }
            if pair_count < k {
                break;
            }
            ans += (n - right + 1) as i64;

            fq.insert(nums[left], *fq.get(&nums[left]).unwrap() - 1);
            pair_count -= *fq.get(&nums[left]).unwrap_or(&0);

            left += 1;
        }

        ans
    }
}

struct Solution;

mod tests {
    use super::*;

    #[test]
    fn test_count_good() {
        assert_eq!(Solution::count_good(vec![3, 1, 4, 3, 2, 2, 4], 2), 4);
        assert_eq!(Solution::count_good(vec![1, 1, 1, 1, 1], 10), 1);
    }
}
