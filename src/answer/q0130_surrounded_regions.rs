// q0130_surrounded_regions

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() {
            return;
        }
        let row = board[0].len();
        let line = board.len();
        let neighber = |(l, r)| {
            let mut v = Vec::with_capacity(4);
            if l != 0 {
                v.push((l - 1, r));
            }
            if l != line - 1 {
                v.push((l + 1, r));
            }
            if r != 0 {
                v.push((l, r - 1));
            }
            if r != row - 1 {
                v.push((l, r + 1));
            }
            println!("{:?}--{:?}", (l, r), v);
            v
        };

        let mut no_overturn = HashSet::<(usize, usize)>::new();
        let mut checked = HashSet::new();
        let mut need_check = vec![];

        macro_rules! handlep {
            (($l: expr, $r: expr)) => {
                let v = neighber(($l, $r));
                checked.insert(($l, $r));
                for p in v.into_iter() {
                    if board[p.0][p.1] == 'O' {
                        no_overturn.insert(p);
                        if !checked.contains(&p) {
                            need_check.push(p);
                        }
                    }
                }
            };
        }

        for (r, c) in board[0].iter().enumerate() {
            if *c == 'O' {
                handlep!((0, r));
            }
        }
        for (r, c) in board[line - 1].iter().enumerate() {
            if *c == 'O' {
                handlep!((line - 1, r));
            }
        }
        for l in 1..line - 1 {
            if board[l][0] != 'O' {
                continue;
            }
            handlep!((l, 0));
        }
        for l in 1..line - 1 {
            if board[l][row - 1] != 'O' {
                continue;
            }
            handlep!((l, row - 1));
        }

        while !need_check.is_empty() {
            let mut nc = vec![];
            std::mem::swap(&mut nc, &mut need_check);
            for pp in nc.into_iter() {
                if checked.contains(&pp) {
                    continue;
                }
                handlep!((pp.0, pp.1));
            }
        }

        for x in 1..line - 1 {
            for y in 1..row - 1 {
                if board[x][y] == 'O' && !no_overturn.contains(&(x, y)) {
                    board[x][y] = 'X';
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut src = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut src);
        assert_eq!(
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X']
            ],
            src
        );

        let mut src = vec![vec!['X']];
        Solution::solve(&mut src);
        assert_eq!(vec![vec!['X']], src);

        let mut src = vec![
            vec!['X', 'O', 'X', 'O', 'X', 'O'],
            vec!['O', 'X', 'O', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'O', 'X', 'O'],
            vec!['O', 'X', 'O', 'X', 'O', 'X'],
        ];
        Solution::solve(&mut src);
        assert_eq!(
            vec![
                vec!['X', 'O', 'X', 'O', 'X', 'O'],
                vec!['O', 'X', 'X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X', 'X', 'O'],
                vec!['O', 'X', 'O', 'X', 'O', 'X']
            ],
            src
        );
    }
}
