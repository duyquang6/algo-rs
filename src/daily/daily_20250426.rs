impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        // ################## SLIDING WINDOW COUNTING #################

        let update_cnt = |state: &mut [i32], value: i32, delta: i32| {
            if value == min_k {
                state[0] += delta;
            }
            if value == max_k {
                state[1] += delta;
            }
        };
        let (mut left, mut right) = (0, 0);
        let mut ans = 0;
        while left < nums.len() {
            // min_right point to the earliest position [left, min_right] form the fixed-bounded subarray
            let mut min_right = nums.len();
            // min and max count
            let mut cnt = [0i32; 2];

            // find min_right
            while right < nums.len() && nums[right] >= min_k && nums[right] <= max_k {
                update_cnt(&mut cnt, nums[right], 1);

                if cnt[0] > 0 && cnt[1] > 0 {
                    min_right = right;
                    break;
                }
                right += 1;
            }

            // extend right
            while right < nums.len() && nums[right] >= min_k && nums[right] <= max_k {
                right += 1;
            }

            // calculate part
            while left < nums.len() && cnt[0] > 0 && cnt[1] > 0 {
                ans += (right - min_right) as i64;

                // move left to one
                update_cnt(&mut cnt, nums[left], -1);
                left += 1;
                // find min_right if cnt_min or cnt_max is zero
                while min_right + 1 < right && (cnt[0] == 0 || cnt[1] == 0) {
                    min_right += 1;
                    update_cnt(&mut cnt, nums[min_right], 1);
                }
            }

            // right is now point to the break point, need to move right to next fixed bound section
            // move left to right, go to next section
            while right < nums.len() && !(nums[right] >= min_k && nums[right] <= max_k) {
                right += 1;
            }
            left = right;
        }

        // ################# OPTIMIZE SLIDING WINDOW ####################
        let mut ans = 0;
        let (mut i_min, mut i_max, mut i_bad) = (-1, -1, -1);
        for right in 0..nums.len() {
            if nums[right] == min_k {
                i_min = right as i32;
            }
            if nums[right] == max_k {
                i_max = right as i32;
            }
            if nums[right] > max_k || nums[right] < min_k {
                i_bad = right as i32;
            }
            ans += 0.max(i_min.min(i_max) - i_bad) as i64;
        }

        ans
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays() {
        assert_eq!(Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2)
    }
}
