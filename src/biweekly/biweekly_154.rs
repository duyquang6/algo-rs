impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }

    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut ans = 1;
        while ans < n {
            ans <<= 1;
        }
        ans
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_operations() {
        assert_eq!(Solution::min_operations(vec![3, 9, 7], 5), 4);
    }
}
