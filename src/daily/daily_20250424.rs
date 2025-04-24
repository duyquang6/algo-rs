use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        // ######### get unique num by sorting #########
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();
        let (_, cnt_uniq) = sorted_nums.iter().fold((0, 0), |(last_value, cnt), &item| {
            if last_value == item {
                return (last_value, cnt);
            }
            (item, cnt + 1)
        });

        // ########## get by using hashset ##############
        let cnt_uniq = nums.iter().collect::<HashSet<&i32>>().len();

        // ######### the brute force way #########
        let mut ans = 0;
        let n = nums.len();
        for i in 0..n {
            let mut hash_table = HashSet::new();
            for j in i..n {
                hash_table.insert(nums[j]);
                if hash_table.len() == cnt_uniq as usize {
                    ans += 1;
                }
            }
        }

        // ######### the sliding way ###########
        let mut ans = 0;
        let n = nums.len();
        let mut right = 0;
        let mut hash_table = HashMap::new();
        for left in 0..n {
            while right < n && hash_table.len() < cnt_uniq {
                hash_table
                    .entry(nums[right])
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                right += 1;
            }
            if hash_table.len() < cnt_uniq {
                break;
            }
            ans += n as i32 - right as i32 + 1;
            hash_table.entry(nums[left]).and_modify(|e| *e -= 1);
            if hash_table.get(&nums[left]).unwrap() == &0 {
                hash_table.remove(&nums[left]);
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
    fn test_count_complete_subarrays() {
        assert_eq!(Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4)
    }
}
