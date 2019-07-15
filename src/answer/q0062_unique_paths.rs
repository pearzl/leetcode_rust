// q0062_unique_paths

struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m * n == 0 {
            return 0;
        }
        let m = m as usize;
        let n = n as usize;
        let mut map = Vec::with_capacity(m);
        for _ in 0..n - 1 {
            map.push(vec![0; m]);
        }
        map.push(vec![1; m]);
        for v in map.iter_mut() {
            v[m - 1] = 1;
        }
        for i in (0..n - 1).rev() {
            for j in (0..m - 1).rev() {
                map[i][j] = &map[i + 1][j] + &map[i][j + 1];
            }
        }
        map[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::unique_paths(3, 2));
        assert_eq!(0, Solution::unique_paths(0, 2));
        assert_eq!(0, Solution::unique_paths(0, 0));
        assert_eq!(0, Solution::unique_paths(3, 0));
        assert_eq!(1, Solution::unique_paths(1, 1));
        assert_eq!(28, Solution::unique_paths(7, 3));
    }
}
