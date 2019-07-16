// q0069_sqrtx

struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }
        let mut l = 1;
        let mut r = 46340; // sqrt(i32::max_value())
        if x >= r * r {
            return r;
        }
        while l != r - 1 {
            let i = (l + r) / 2;
            let t = i * i;
            if x == t {
                return i;
            } else if x > t {
                l = i;
            } else {
                r = i;
            }
        }
        return l;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::my_sqrt(2147395600), 46340);

        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(2), 1);
        assert_eq!(Solution::my_sqrt(3), 1);
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(5), 2);
        assert_eq!(Solution::my_sqrt(6), 2);
        assert_eq!(Solution::my_sqrt(7), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(9), 3);
        assert_eq!(Solution::my_sqrt(10), 3);
        assert_eq!(Solution::my_sqrt(11), 3);
        assert_eq!(Solution::my_sqrt(12), 3);
        assert_eq!(Solution::my_sqrt(13), 3);
        assert_eq!(Solution::my_sqrt(14), 3);
        assert_eq!(Solution::my_sqrt(15), 3);
        assert_eq!(Solution::my_sqrt(16), 4);
        assert_eq!(Solution::my_sqrt(17), 4);
        assert_eq!(Solution::my_sqrt(18), 4);
        assert_eq!(Solution::my_sqrt(19), 4);
        assert_eq!(Solution::my_sqrt(20), 4);
        assert_eq!(Solution::my_sqrt(21), 4);
        assert_eq!(Solution::my_sqrt(22), 4);
        assert_eq!(Solution::my_sqrt(23), 4);
        assert_eq!(Solution::my_sqrt(24), 4);
        assert_eq!(Solution::my_sqrt(25), 5);
        assert_eq!(Solution::my_sqrt(26), 5);
    }
}
