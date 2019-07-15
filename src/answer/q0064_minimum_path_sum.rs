// q0064_minimum_path_sum

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        if n == 0 {
            return 0;
        }
        let mut grid = grid;
        for i in (0..m - 1).rev() {
            grid[i][n - 1] += grid[i + 1][n - 1];
        }
        for i in (0..n - 1).rev() {
            grid[m - 1][i] += grid[m - 1][i + 1];
        }
        for i in (0..m - 1).rev() {
            for j in (0..n - 1).rev() {
                grid[i][j] += grid[i + 1][j].min(grid[i][j + 1]);
            }
        }
        grid[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            7,
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
    }
}
