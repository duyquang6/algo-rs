impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut fq = vec![0; 40];

        for i in 1..=n {
            fq[get_decimal_sum(i) as usize] += 1;
        }

        let max = fq.iter().max().unwrap();

        fq.iter()
            .map(|f| {
                if f == max {
                    return 1;
                }
                0
            })
            .sum()
    }
}

fn get_decimal_sum(mut n: i32) -> i32 {
    let mut ans = 0;
    while n > 0 {
        ans += n % 10;
        n /= 10;
    }

    ans
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_largest_group() {
        assert_eq!(Solution::count_largest_group(13), 4)
    }
}
