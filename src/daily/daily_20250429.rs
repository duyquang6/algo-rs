impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let &max_num = nums.iter().max().unwrap();
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = 0;

        for right in 0..nums.len() {
            if nums[right] == max_num {
                cnt += 1;
            }
            while cnt >= k && left <= right {
                if nums[left] == max_num {
                    cnt -= 1;
                }
                left += 1;
            }

            ans += right as i64 - left as i64 + 1;
        }

        (nums.len() as i64 * (nums.len() as i64 + 1)) / 2 - ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays() {
        println!("{}", Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2))
    }
}
