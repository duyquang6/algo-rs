use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let n = arr.len();
        let mut ans = 0i32;
        for i in 0..n - 2 {
            let vi = arr[i];
            for j in i + 1..n - 1 {
                let vj = arr[j];

                for &vk in &arr[j + 1..n] {
                    if (vi - vj).abs() <= a && (vj - vk).abs() <= b && (vi - vk).abs() <= c {
                        ans += 1;
                    }
                }
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
    fn test_count_good_triplets() {
        assert_eq!(
            Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3),
            4
        );
    }
}
