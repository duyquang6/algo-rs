struct TicTacToe {
    // boards: Vec<Vec<u8>>,
    n: i32,
    rows_count: Vec<[u8; 2]>,
    cols_count: Vec<[u8; 2]>,
    diag_count: [u8; 2],
    anti_diag_count: [u8; 2],
    done: bool,
}

impl TicTacToe {
    fn new(n: i32) -> Self {
        Self {
            n,
            rows_count: vec![[0, 0]; n as usize],
            cols_count: vec![[0, 0]; n as usize],
            diag_count: [0, 0],
            anti_diag_count: [0, 0],
            done: false,
        }
    }

    fn _move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        if self.done {
            return -1;
        }

        let (row, col) = (row as usize, col as usize);

        let player_idx = player as usize - 1;
        self.rows_count[row][player_idx] += 1;
        self.cols_count[col][player_idx] += 1;
        if row == col {
            self.diag_count[player_idx] += 1;
        }
        if row == self.n as usize - col - 1 {
            self.anti_diag_count[player_idx] += 1;
        }

        if [
            self.rows_count[row][player_idx],
            self.cols_count[col][player_idx],
            self.diag_count[player_idx],
            self.anti_diag_count[player_idx],
        ]
        .iter()
        .any(|&count| count == self.n as u8)
        {
            player
        } else {
            0
        }
    }
}
