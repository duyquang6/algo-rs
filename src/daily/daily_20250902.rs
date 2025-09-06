use std::{cmp::Reverse, collections::*};

struct Solution;

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for (i1, p1) in points.iter().enumerate() {
            for (i2, p2) in points.iter().enumerate() {
                if i1 == i2 {
                    continue;
                }

                if !check_upper_left(p1, p2) {
                    continue;
                }

                let mut inside = false;
                for (i3, p3) in points.iter().enumerate() {
                    if i3 == i1 || i3 == i2 {
                        continue;
                    }

                    if is_inside(p1, p2, p3) {
                        inside = true;
                        break;
                    }
                }

                if !inside {
                    ans += 1;
                }
            }
        }

        ans
    }
}

fn is_inside(upper_left: &[i32], bottom_right: &[i32], p: &[i32]) -> bool {
    return p[0] >= upper_left[0]
        && p[0] <= bottom_right[0]
        && p[1] >= bottom_right[1]
        && p[1] <= upper_left[1];
}

fn check_upper_left(a: &[i32], b: &[i32]) -> bool {
    return a[0] <= b[0] && a[1] >= b[1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_pairs() {
        assert_eq!(
            Solution::number_of_pairs(vec![vec![3, 1], vec![1, 3,], vec![1, 1]]),
            2
        );
    }
}
