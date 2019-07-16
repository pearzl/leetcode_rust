// q0073_set_matrix_zeroes

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut buf = None;
        for (i, line) in matrix.iter().enumerate() {
            for (j, v) in line.iter().enumerate() {
                if *v == 0 {
                    buf = Some((i, j));
                    break;
                }
            }
            if buf.is_some() {
                break;
            }
        }
        if buf.is_none() {
            return;
        }
        let (x, y) = buf.unwrap();
        for i in 0..matrix.len() {
            if i == x {
                continue;
            }
            for j in 0..matrix[i].len() {
                if j == y {
                    continue;
                }
                if matrix[i][j] == 0 {
                    matrix[i][y] = 0;
                    matrix[x][j] = 0;
                }
            }
        }
        for i in 0..matrix.len() {
            if i == x {
                continue;
            }
            for j in 0..matrix[i].len() {
                if j == y {
                    continue;
                }
                if matrix[i][y] == 0 {
                    matrix[i][j] = 0;
                } else if matrix[x][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if j == y || x == i {
                    matrix[i][j] = 0;
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
        let mut t = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut t);
        assert_eq!(t, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);

        let mut t = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut t);
        assert_eq!(
            t,
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
        );
    }
}
