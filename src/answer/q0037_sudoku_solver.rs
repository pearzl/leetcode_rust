// q0037_sudoku_solver

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        assert!(Solution::solve(0, 0, board));
        // Solution::solve(0, 0, board);
    }

    fn solve(mut x: usize, mut y: usize, v: &mut Vec<Vec<char>>) -> bool {
        loop {
            if v[x][y] == '.' {
                let pos_v = Solution::possible_value(x, y, v);
                // println!("position [{},{}] has possible value: {:?}", x, y, pos_v);
                if pos_v.len() == 0 {
                    return false;
                }
                for k in pos_v.iter() {
                    // println!("position [{},{}] insert value {}", x, y, *k);
                    v[x][y] = *k;
                    let r = Solution::solve(x, y, v);
                    if r {
                        return true;
                    }
                }
                v[x][y] = '.';
                return false;
            }
            y += 1;
            if y == 9 {
                y = 0;
                x += 1;
                if x == 9 {
                    break;
                }
            }
        }
        true
    }

    pub fn possible_value(i: usize, j: usize, v: &Vec<Vec<char>>) -> Vec<char> {
        let mut buf = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
        for n in v[i].iter() {
            if *n == '.' {
                continue;
            }
            let n = (*n as u8 - b'1') as usize;
            buf[n] = '.';
        }
        for line in v.iter() {
            if line[j] == '.' {
                continue;
            }
            let n = (line[j] as u8 - b'1') as usize;
            buf[n] = '.';
        }
        let xr = i / 3 * 3;
        let yr = j / 3 * 3;
        for x in xr..xr + 3 {
            for y in yr..yr + 3 {
                if v[x][y] == '.' {
                    continue;
                }
                let n = (v[x][y] as u8 - b'1') as usize;
                buf[n] = '.';
            }
        }
        buf.iter().filter(|x| **x != '.').cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util;

    #[test]
    fn it_works() {
        let mut sudo = util::build_sudo([
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ]);
        let ret = util::build_sudo([
            ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
            ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
            ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
            ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
            ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
            ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
            ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
            ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
            ["3", "4", "5", "2", "8", "6", "1", "7", "9"],
        ]);
        Solution::solve_sudoku(&mut sudo);
        assert_eq!(sudo, ret);
        util::print_sudo(sudo);
    }

    #[test]
    #[ignore]
    fn test_possible_value() {
        let sudo = util::build_sudo([
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ]);
        assert_eq!(vec!['1', '2', '4'], Solution::possible_value(0, 2, &sudo));
        assert_eq!(
            vec!['1', '2', '4', '5'],
            Solution::possible_value(8, 1, &sudo)
        );
        assert_eq!(
            vec!['4', '5', '8', '9'],
            Solution::possible_value(5, 6, &sudo)
        );
    }
}
