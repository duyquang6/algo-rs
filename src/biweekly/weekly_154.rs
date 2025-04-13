impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
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
