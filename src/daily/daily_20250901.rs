use std::{cmp::Reverse, collections::*};

struct Solution;

impl Solution {
    pub fn max_average_ratio(mut classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
        let mut pq = BinaryHeap::new();

        for (idx, class) in classes.iter().enumerate() {
            if class[0] == class[1] {
                continue;
            }

            let cur = class[0] as f64 / class[1] as f64;
            let next = (class[0] + 1) as f64 / (class[1] + 1) as f64;

            pq.push((((next - cur) * 1e9) as i64, idx));
        }

        while (extra_students > 0 && !pq.is_empty()) {
            let (score, idx) = pq.pop().unwrap();
            let class = &mut classes[idx];
            class[0] += 1;
            class[1] += 1;

            let cur = class[0] as f64 / class[1] as f64;
            let next = (class[0] + 1) as f64 / (class[1] + 1) as f64;
            pq.push((((next - cur) * 1e9) as i64, idx));

            extra_students -= 1;
        }

        classes
            .iter()
            .map(|v| v[0] as f64 / v[1] as f64)
            .sum::<f64>()
            / classes.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_average_ratio() {
        assert_eq!(
            Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
            0.7833333333333333
        );
    }
}
