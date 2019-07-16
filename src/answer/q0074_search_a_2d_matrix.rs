// q0074_search_a_2d_matrix

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 {
            return false;
        }
        if matrix[0].len() == 0 {
            return false;
        }
        let mut target_line: i32 = -1;
        for v in matrix.iter() {
            if v[0] == target {
                return true;
            } else if v[0] < target {
                target_line += 1;
            } else {
                break;
            }
        }
        if target_line < 0 {
            return false;
        }
        matrix[target_line as usize][1..].contains(&target)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                3
            )
        );

        assert_eq!(
            false,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                13
            )
        );

        let empty: Vec<i32> = vec![];
        assert_eq!(false, Solution::search_matrix(vec![empty], 13));

        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(false, Solution::search_matrix(empty, 13));

        assert_eq!(true, Solution::search_matrix(vec![vec![13]], 13));
        assert_eq!(false, Solution::search_matrix(vec![vec![14]], 13));
        assert_eq!(false, Solution::search_matrix(vec![vec![12],vec![14]], 13));
        assert_eq!(true, Solution::search_matrix(vec![vec![13],vec![14]], 13));
    }
}
