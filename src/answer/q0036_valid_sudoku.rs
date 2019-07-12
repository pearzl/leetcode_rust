// q0036_valid_sudoku

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut line_checker: [[bool; 9]; 9] = [[false; 9]; 9];
        let mut row_checker: [[bool; 9]; 9] = [[false; 9]; 9];
        let mut block_checker: [[bool; 9]; 9] = [[false; 9]; 9];
        for (i, line) in board.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '.' {
                    continue;
                }
                let c = (*c as u8 - b'1') as usize;
                // if i==4 && c == 4 {
                //     // println!("{}---{}", i, j);
                // }
                if line_checker[i][c] {
                    // println!("linec: {}-{}", i, c);
                    return false;
                } else {
                    line_checker[i][c] = true;
                }
                if row_checker[j][c] {
                    // println!("rowc: {}-{}", j, c);
                    return false;
                } else {
                    row_checker[j][c] = true;
                }
                let n = (i / 3) * 3 + (j / 3);
                if block_checker[n][c] {
                    // println!("blockc: {}-{}", n, c);
                    return false;
                } else {
                    block_checker[n][c] = true;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util;
    
    #[test]
    fn it_works() {
        // assert_eq!(
        //     true,
        //     Solution::is_valid_sudoku(util::build([
        //         ["5", "3", ".", ".", "7", ".", ".", ".", "."],
        //         ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        //         [".", "9", "8", ".", ".", ".", ".", "6", "."],
        //         ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        //         ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        //         ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        //         [".", "6", ".", ".", ".", ".", "2", "8", "."],
        //         [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        //         [".", ".", ".", ".", "8", ".", ".", "7", "9"]
        //     ]))
        // );
        // assert_eq!(
        //     false,
        //     Solution::is_valid_sudoku(util::build([
        //         ["8", "3", ".", ".", "7", ".", ".", ".", "."],
        //         ["6", ".", ".", "1", "9", "5", ".", ".", "."],
        //         [".", "9", "8", ".", ".", ".", ".", "6", "."],
        //         ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
        //         ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
        //         ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
        //         [".", "6", ".", ".", ".", ".", "2", "8", "."],
        //         [".", ".", ".", "4", "1", "9", ".", ".", "5"],
        //         [".", ".", ".", ".", "8", ".", ".", "7", "9"]
        //     ]))
        // );
        assert_eq!(
            true,
            Solution::is_valid_sudoku(util::build([
                [".", "8", "7", "6", "5", "4", "3", "2", "1"],
                ["2", ".", ".", ".", ".", ".", ".", ".", "."],
                ["3", ".", ".", ".", ".", ".", ".", ".", "."],
                ["4", ".", ".", ".", ".", ".", ".", ".", "."],
                ["5", ".", ".", ".", ".", ".", ".", ".", "."],
                ["6", ".", ".", ".", ".", ".", ".", ".", "."],
                ["7", ".", ".", ".", ".", ".", ".", ".", "."],
                ["8", ".", ".", ".", ".", ".", ".", ".", "."],
                ["9", ".", ".", ".", ".", ".", ".", ".", "."]
            ]))
        );
    }
}
