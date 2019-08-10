// q0118_pascals_triangle

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {
            return vec![];
        }
        let mut ret = Vec::with_capacity(num_rows as usize);
        ret.push(vec![1]);
        for line in 2..=num_rows {
            let line = line as usize;
            let last_line = ret.last().unwrap();
            let mut tv = Vec::with_capacity(line);
            tv.push(last_line[0]);
            for i in 1..line - 1 {
                let t = last_line[i - 1] + last_line[i];
                tv.push(t);
            }
            tv.push(last_line[line - 2]);
            ret.push(tv);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );

        assert_eq!(Solution::generate(1), vec![vec![1]]);

        assert_eq!(Solution::generate(0), Vec::<Vec<i32>>::new());
    }
}
