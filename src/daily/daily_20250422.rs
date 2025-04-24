impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        // ############### COMBINATORIC MATH ################
        const MOD: i64 = 1e9 as i64 + 7;
        const MAX_N: usize = 10_000 + 10;
        const MAX_P: usize = 15;

        let mut sieve = vec![0; MAX_N];
        for i in 2..MAX_N {
            if sieve[i] == 0 {
                for j in (i..MAX_N).step_by(i) {
                    sieve[j] = i as i32;
                }
            }
        }

        let mut ps = vec![vec![]; MAX_N];
        for i in 2..=max_value {
            let mut x = i;
            while x > 1 {
                let p = sieve[x as usize];
                let mut cnt = 0;
                while x % p == 0 {
                    cnt += 1;
                    x /= p;
                }
                ps[i as usize].push(cnt);
            }
        }

        let mut c = vec![vec![0; MAX_P + 1]; n as usize + MAX_P + 1];
        c[0][0] = 1;

        for i in 1..=n as usize + MAX_P {
            c[i][0] = 1;
            for j in 1..=i.min(MAX_P) {
                c[i][j] = (c[i - 1][j] + c[i - 1][j - 1]) % MOD;
            }
        }

        let mut ans = 0i64;
        let n = n as usize;
        for x in 1..=max_value as usize {
            let mut mul = 1i64;
            for &p in &ps[x] {
                mul = mul * c[n + p as usize - 1][p as usize] % MOD;
            }
            ans = (ans + mul) % MOD;
        }

        // ################ DP ###################
        // TODO
        // let mut dp = vec![vec![0; MAX_P]; max_value as usize + 1];
        // for i in 1..=max_value as usize {
        //     dp[i][1] = 1;
        //     let mut j = 2;
        //     while i * j <= max_value as usize {
        //         for k in 1..MAX_P - 1 {
        //             dp[i * j][k + 1] += dp[i][k];
        //         }
        //         j += 1;
        //     }
        // }
        // https://stackoverflow.com/questions/64997030/compute-binomial-coefficient-module-prime-p-in-c

        // let mut ans = 0i64;

        ans as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ideal_arrays() {
        assert_eq!(Solution::ideal_arrays(2, 5), 10)
    }
}
