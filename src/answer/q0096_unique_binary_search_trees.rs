// q0096_unique_binary_search_trees

struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // if n == 0 {
        //     return 0;
        // }else {
        //     return Solution::num_trees(n) +
        // }
        let n = n as usize;
        let mut buf = vec![0 as i32; n + 1];
        buf[0] = 0;
        buf[1] = 1;
        for i in 2..=n {
            for j in 1..=i {
                let r1 = &buf[j - 1];
                let r2 = &buf[i - j];
                let nr = r1 * r2;
                if nr == 0 {
                    buf[i] += r1 + r2;
                } else {
                    buf[i] += nr;
                }
            }
        }
        buf[n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::num_trees(2));
        assert_eq!(5, Solution::num_trees(3));
        assert_eq!(14, Solution::num_trees(4));
        assert_eq!(42, Solution::num_trees(5));
        assert_eq!(42, Solution::num_trees(5));
        assert_eq!(16796, Solution::num_trees(10));
    }
}
