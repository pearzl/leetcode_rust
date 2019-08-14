// q0120_triangle

struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.is_empty() {
            return 0;
        }
        let mut triangle = triangle;
        let mut buf = triangle.pop().unwrap();
        for mut level in triangle.into_iter().rev() {
            for (i, min_step) in level.iter_mut().enumerate() {
                let t1 = *min_step + buf[i];
                let t2 = *min_step + buf[i + 1];
                buf[i] = t1.min(t2);
            }
        }
        return buf[0];
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            11,
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
        );
        assert_eq!(0, Solution::minimum_total(Vec::<Vec<i32>>::new()));
    }
}
