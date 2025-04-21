impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut prefix_sum = vec![0];
        prefix_sum.extend(differences.iter().scan(0i64, |acc, &x| {
            *acc += x as i64;
            Some(*acc)
        }));
        let &min = prefix_sum.iter().min().unwrap();
        let &max = prefix_sum.iter().max().unwrap();

        let delta_lower = lower as i64 - min;
        let max_lowest = delta_lower + max;

        return (upper as i64 - max_lowest + 1).max(0) as i32;
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_arrays() {
        assert_eq!(Solution::number_of_arrays(vec![3, -4, 5, 1, -2], -4, 5), 4)
    }
}
