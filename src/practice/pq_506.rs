use std::{cmp::Reverse, collections::*};
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        // Reverse
        let mut max_heap = BinaryHeap::new();
        for (i, n) in score.iter().enumerate() {
            max_heap.push((*n, i));
        }

        let mut ans = vec!["".to_string(); score.len()];

        for i in 0..score.len() {
            let (_, idx) = max_heap.pop().unwrap();

            ans[idx] = match i {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (i + 1).to_string(),
            }
        }

        ans
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_good_numbers() {
        assert_eq!(
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
            ["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
        );
    }
}
