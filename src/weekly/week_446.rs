impl Solution {
    pub fn calculate_score(instructions: Vec<String>, values: Vec<i32>) -> i64 {
        let n = instructions.len();
        let mut visited = vec![false; n];
        let mut ans = 0;
        let mut idx = 0i32;
        loop {
            if idx < 0 || idx >= n as i32 {
                break;
            }
            if visited[idx as usize] {
                break;
            }
            visited[idx as usize] = true;

            if instructions[idx as usize] == "jump" {
                idx += values[idx as usize];
            } else {
                ans += values[idx as usize] as i64;
                idx += 1;
            }
        }

        ans
    }
    pub fn maximum_possible_size(nums: Vec<i32>) -> i32 {
        // let mut stack = Vec::new();
        let mut ans = 0;

        let mut max_so_far = 0;

        for &n in &nums {
            max_so_far = n.max(max_so_far);
            if n < max_so_far {
                ans += 1;
            }
        }

        nums.len() as i32 - ans
    }

    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        vec![]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // fn test_result_array() {
    //     assert_eq!(
    //         Solution::result_array(vec![1, 2, 3, 4, 5], 3),
    //         vec![9, 2, 4]
    //     )
    // }
    #[test]
    fn test_maximum_possible_size() {
        assert_eq!(Solution::maximum_possible_size(vec![4, 2, 5, 3, 5],), 3)
    }

    #[test]
    fn test_calculate_score() {
        assert_eq!(
            Solution::calculate_score(
                vec!["jump", "add", "add", "jump", "add", "jump"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect(),
                vec![2, 1, 3, 1, -2, -3],
            ),
            1
        )
    }
}
