impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for i in 0..n {
            for j in i + 1..n {
                if nums[i] == nums[j] && (i as i32 * j as i32) % k == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn gcd_recur(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

struct Solution;

mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 6), 2);
        assert_eq!(gcd_recur(2, 6), 2);
    }

    #[test]
    fn test_count_good() {
        assert_eq!(Solution::count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2), 4);
        assert_eq!(Solution::count_pairs(vec![1, 2, 3, 4], 1), 0);
    }
}
