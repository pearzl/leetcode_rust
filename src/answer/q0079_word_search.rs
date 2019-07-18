// q0079_word_search

struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let ref mut used_pos = vec![];
        let mut pos_pos = vec![];
        for (i, line) in board.iter().enumerate() {
            for (j, _) in line.iter().enumerate() {
                pos_pos.push((i, j));
            }
        }
        Solution::find_from(used_pos, &word[..], pos_pos, &board)
    }

    fn find_from(
        used_pos: &mut Vec<(usize, usize)>,
        word: &str,
        pos_pos: Vec<(usize, usize)>,
        board: &Vec<Vec<char>>,
    ) -> bool {
        if word == "" {
            return true;
        }
        let first_char = word.chars().next().unwrap();
        for p in pos_pos.into_iter() {
            if first_char == board[p.0][p.1] {
                let mut nc_pos = vec![];
                if p.0 != 0 {
                    nc_pos.push((p.0 - 1, p.1));
                }
                if p.0 != board.len() - 1 {
                    nc_pos.push((p.0 + 1, p.1));
                }
                if p.1 != 0 {
                    nc_pos.push((p.0, p.1 - 1));
                }
                if p.1 != board[0].len() - 1 {
                    nc_pos.push((p.0, p.1 + 1));
                }
                let nc_pos: Vec<(usize, usize)> = nc_pos
                    .into_iter()
                    .filter(|pp| !used_pos.contains(pp))
                    .collect();
                // if nc_pos.is_empty() {
                //     return false;
                // }
                used_pos.push(p);
                if Solution::find_from(used_pos, &word[1..], nc_pos, board) {
                    return true;
                }
                used_pos.pop();
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        assert_eq!(true, Solution::exist(board.clone(), String::from("ABCCED")));
        assert_eq!(true, Solution::exist(board.clone(), String::from("SEE")));
        assert_eq!(false, Solution::exist(board.clone(), String::from("ABCB")));
        assert_eq!(false, Solution::exist(board.clone(), String::from("ABCB")));
        assert_eq!(true, Solution::exist(board.clone(), String::from("A")));
        assert_eq!(true, Solution::exist(vec![vec!['A']], String::from("A")));
    }
}
