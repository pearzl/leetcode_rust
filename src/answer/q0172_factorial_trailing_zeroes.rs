// q0172_factorial_trailing_zeroes 


struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut ret = 0;
        let mut n = n;
        while n >= 5 {
            n /= 5;
            ret += n;
        }
        ret
    }
}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!( 0, Solution::trailing_zeroes(3));
        assert_eq!( 1, Solution::trailing_zeroes(5));
        assert_eq!( 2, Solution::trailing_zeroes(10));
        assert_eq!( 3, Solution::trailing_zeroes(15));
        assert_eq!( 4, Solution::trailing_zeroes(20));
        assert_eq!( 6, Solution::trailing_zeroes(25));
        assert_eq!( 7, Solution::trailing_zeroes(30));
        assert_eq!( 8, Solution::trailing_zeroes(35));
        assert_eq!( 9, Solution::trailing_zeroes(40));
        assert_eq!( 10, Solution::trailing_zeroes(45));
        assert_eq!( 12, Solution::trailing_zeroes(50));
        assert_eq!( 21, Solution::trailing_zeroes(90));
        assert_eq!( 24, Solution::trailing_zeroes(100));
        assert_eq!( 452137076, Solution::trailing_zeroes(1808548329));
    }
}

