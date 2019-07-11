// q0029_divide_two_integers

struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::min_value() && divisor == -1 {
            return i32::max_value();
        }
        if dividend == i32::min_value() && divisor == 1 {
            return i32::min_value();
        }
        let f1 = dividend < 0;
        let f2 = divisor < 0;
        let neg = f1 != f2;
        let mut dividend = if f1 { dividend } else { 0 - dividend };
        let mut divisor = if f2 { divisor } else { 0 - divisor };
        let mut bn = 0;
        let mut d = divisor;
        let mut r = 0;
        while d >= dividend - d {
            bn += 1;
            d <<= 1;
        }
        while dividend <= divisor {
            if dividend == divisor {
                r += (1 << bn);
                break;
            }
            // println!("{},{},{}", dividend, d, bn);
            dividend -= d;
            r += (1 << bn);
            if dividend == 0 {
                break;
            }
            d >>= 1;
            bn -= 1;
            while d < dividend {
                d >>= 1;
                bn -= 1;
            }
        }
        if neg {
            return 0 - r;
        } else {
            return r;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::divide(10, 3));
        assert_eq!(-2, Solution::divide(7, -3));
        assert_eq!(1, Solution::divide(1, 1));
        assert_eq!(-1, Solution::divide(1, -1));
        assert_eq!(1, Solution::divide(-1, -1));
        assert_eq!(2147483647, Solution::divide(-2147483648, -1));
        assert_eq!(1073741824, Solution::divide(-2147483648, -2));
        assert_eq!(-2147483648, Solution::divide(-2147483648, 1));
        assert_eq!(-1073741824, Solution::divide(-2147483648, 2));
    }
}
