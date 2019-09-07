// q0200_number_of_islands

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let line = grid.len();
        if line == 0 {
            return 0;
        }
        let row = grid[0].len();
        let mut checked = vec![vec![false; row]; line];
        let mut num = 0;
        let check_point = |l, r| {
            let mut need_check = vec![];
            if l != 0 {
                need_check.push((l - 1, r));
            }
            if l != line - 1 {
                need_check.push((l + 1, r));
            }
            if r != row - 1 {
                need_check.push((l, r + 1));
            }
            if r != 0 {
                need_check.push((l, r - 1));
            }
            need_check
        };
        for l in 0..line {
            for r in 0..row {
                if checked[l][r] {
                    continue;
                }
                checked[l][r] = true;
                if grid[l][r] == '1' {
                    num += 1;
                    let mut need_check = check_point(l, r);
                    while let Some(p) = need_check.pop() {
                        if checked[p.0][p.1] {
                            continue;
                        }
                        checked[p.0][p.1] = true;
                        if grid[p.0][p.1] == '1' {
                            need_check.append(&mut check_point(p.0, p.1));
                        }
                    }
                }
            }
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!(
        //     1,
        //     Solution::num_islands(vec![
        //         vec!['1', '1', '1', '1', '0'],
        //         vec!['1', '1', '0', '1', '0'],
        //         vec!['1', '1', '0', '0', '0'],
        //         vec!['0', '0', '0', '0', '0']
        //     ])
        // );
        // assert_eq!(
        //     3,
        //     Solution::num_islands(vec![
        //         vec!['1', '1', '0', '0', '0'],
        //         vec!['1', '1', '0', '0', '0'],
        //         vec!['0', '0', '1', '0', '0'],
        //         vec!['0', '0', '0', '1', '1']
        //     ])
        // );
        assert_eq!(
            1,
            Solution::num_islands(vec![
                vec!['1', '1', '1'],
                vec!['0', '1', '0'],
                vec!['1', '1', '1']
            ])
        );
    }
}
