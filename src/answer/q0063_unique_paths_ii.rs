// q0063_unique_paths_ii

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut og = obstacle_grid;
        let line = og.len();
        if line == 0 {
            return 0;
        }
        let row = og[0].len();
        if og[line - 1][row - 1] == 1 {
            return 0;
        }
        og[line - 1][row - 1] = 1;
        for i in (0..line - 1).rev() {
            if og[i][row - 1] == 1 {
                og[i][row - 1] = 0
            } else {
                og[i][row - 1] = og[i + 1][row - 1];
            }
        }
        for i in (0..row - 1).rev() {
            if og[line - 1][i] == 1 {
                og[line - 1][i] = 0;
            } else {
                og[line - 1][i] = og[line - 1][i + 1];
            }
        }
        for i in (0..line - 1).rev() {
            for j in (0..row - 1).rev() {
                if og[i][j] == 1 {
                    og[i][j] = 0;
                } else {
                    og[i][j] = og[i + 1][j] + og[i][j + 1];
                }
            }
        }
        og[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!(
        //     2,
        //     Solution::unique_paths_with_obstacles(vec![
        //         vec![0, 0, 0],
        //         vec![0, 1, 0],
        //         vec![0, 0, 0]
        //     ])
        // );
        // let empty: Vec<Vec<i32>> = vec![];
        // assert_eq!(0, Solution::unique_paths_with_obstacles(empty));
        // assert_eq!(1, Solution::unique_paths_with_obstacles(vec![vec![0, 0,]]));
        // assert_eq!(0, Solution::unique_paths_with_obstacles(vec![vec![1]]));
        // assert_eq!(1, Solution::unique_paths_with_obstacles(vec![vec![0]]));
        assert_eq!(
            0,
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![1, 1], vec![0, 0]])
        );
        assert_eq!(
            0,
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 1]])
        );
    }
}
