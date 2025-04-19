impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        if nums.len() == 1 {
            return 0;
        }
        nums.sort_unstable();

        count_fair_under(&nums, upper) - count_fair_under(&nums, lower - 1)
    }
}

fn count_fair_under(nums: &Vec<i32>, upper: i32) -> i64 {
    let mut right = nums.len() - 1;
    let mut cnt = 0;

    for left in 0..nums.len() {
        while right > left && nums[left] + nums[right] > upper {
            right -= 1;
        }
        if right <= left {
            break;
        }
        cnt += (right - left) as i64
    }
    cnt
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_fair_pairs() {
        assert_eq!(Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6)
    }
}
