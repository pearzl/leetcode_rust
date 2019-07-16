// q0066_plus_one

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let l = digits.len();
        if l == 0 {
            return vec![1];
        }
        let mut flag = 0;
        let mut digits = digits;
        digits[l - 1] += 1;
        for i in digits.iter_mut().rev() {
            let t = *i + flag;
            flag = t / 10;
            *i = t % 10;
        }
        if flag == 1 {
            digits.insert(0, 1);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
        assert_eq!(vec![1], Solution::plus_one(vec![0]));
        assert_eq!(vec![1, 3, 0], Solution::plus_one(vec![1, 2, 9]));
        assert_eq!(vec![2, 0, 0], Solution::plus_one(vec![1, 9, 9]));
        assert_eq!(vec![1, 0, 0, 0], Solution::plus_one(vec![9, 9, 9]));
    }
}
