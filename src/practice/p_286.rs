struct Solution;

impl Solution {
    pub fn wall_and_gates(rooms: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut memo = vec![vec![-2; rooms[0].len()]; rooms.len()];
        let mut visited = vec![vec![false; rooms[0].len()]; rooms.len()];

        for row in 0..rooms.len() as i32 {
            for col in 0..rooms[0].len() as i32 {
                recur(&rooms, &mut visited, &mut memo, row, col);
            }
        }

        for row in 0..rooms.len() {
            for col in 0..rooms[0].len() {
                if memo[row][col] == -2 {
                    memo[row][col] = 2147483647
                }
            }
        }

        memo
    }
}
const DIRS: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

fn recur(
    rooms: &Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    memo: &mut Vec<Vec<i32>>,
    row: i32,
    col: i32,
) -> i32 {
    if row < 0 || row >= rooms.len() as i32 || col < 0 || col >= rooms[0].len() as i32 {
        return 250 * 250;
    }

    if memo[row as usize][col as usize] != -2 {
        return memo[row as usize][col as usize];
    }

    if rooms[row as usize][col as usize] == -1 {
        memo[row as usize][col as usize] = -1;
        return -1;
    }

    if rooms[row as usize][col as usize] == 0 {
        memo[row as usize][col as usize] = 0;
        return 0;
    }

    visited[row as usize][col as usize] = true;

    let mut ans = 2147483647;

    for [dx, dy] in DIRS {
        let (nx, ny) = (row + dx, col + dy);
        if nx < 0
            || nx >= rooms.len() as i32
            || ny < 0
            || ny >= rooms[0].len() as i32
            || visited[nx as usize][ny as usize]
        {
            continue;
        }

        let res = recur(rooms, visited, memo, nx, ny);
        if res == -1 || res == 2147483647 {
            continue;
        }
        ans = ans.min(res + 1);
    }

    visited[row as usize][col as usize] = false;

    if ans == 2147483647 {
        return 2147483647;
    }
    memo[row as usize][col as usize] = ans;
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_and_gates() {
        assert_eq!(
            Solution::wall_and_gates(vec![
                vec![2147483647, -1, 0, 2147483647],
                vec![2147483647, 2147483647, 2147483647, -1],
                vec![2147483647, -1, 2147483647, -1],
                vec![0, -1, 2147483647, 2147483647],
            ]),
            vec![[3, -1, 0, 1], [2, 2, 1, -1], [1, -1, 2, -1], [0, -1, 3, 4]]
        );

        assert_eq!(Solution::wall_and_gates(vec![vec![-1],]), vec![[-1]]);
    }
}
