use std::collections::HashMap;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let nums: Vec<i32> = nums
            .iter()
            .map(|&n| if n % modulo == k { 1 } else { 0 })
            .collect();
        let mut hash_table = HashMap::new();
        let mut cumsum = 0;
        hash_table.insert(0, 1);
        let mut ans = 0;
        for n in nums {
            cumsum += n;
            let target = (cumsum - k) % modulo;
            ans += hash_table.get(&target).unwrap_or(&0);

            hash_table
                .entry(cumsum % modulo)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_interesting_subarrays() {
        assert_eq!(
            Solution::count_interesting_subarrays(vec![3, 2, 4], 2, 1),
            3
        )
    }
}
