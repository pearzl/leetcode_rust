// q0050_powx_n

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if x == 0.0 {
            return 0.0;
        }
        if n == 1 {
            return x;
        } else if n == -1 {
            return 1.0 / x;
        }
        if n == i32::min_value() {
            return 1.0_f64 / (x * Solution::my_pow(x, i32::max_value()));
        }

        let m = n.abs() as usize;
        let mut buf = vec![0.0; m + 1];
        let r = Solution::solve(x, m as usize, &mut buf);
        if n < 0 {
            1.0_f64 / r
        } else {
            r
        }
    }

    fn solve(x: f64, n: usize, buf: &mut Vec<f64>) -> f64 {
        if n == 0 {
            return 1.0;
        } else if n == 1 {
            return x;
        }
        if buf[n] == 0.0 {
            let hf = Solution::solve(x, n / 2, buf);
            if n % 2 == 1 {
                buf[n] = x * hf * hf;
            } else {
                buf[n] = hf * hf;
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
        assert_eq!(Solution::my_pow(2.00000, 10), 2.00000_f64.powi(10));
        assert_eq!(Solution::my_pow(2.10000, 3), 2.10000_f64.powi(3));
        assert_eq!(Solution::my_pow(2.00000, -2), 2.00000_f64.powi(-2));
        assert_eq!(Solution::my_pow(2.00000, 1), 2.00000_f64.powi(1));
        assert_eq!(Solution::my_pow(2.00000, 0), 2.00000_f64.powi(0));
        assert_eq!(Solution::my_pow(0.0, 10), 0.0_f64.powi(10));
        // assert_eq!( Solution::my_pow(0.00001, 2147483647), 0.00001_f64.powi(2147483647));
        assert_eq!(Solution::my_pow(0.00001, 2147483647), 0.0);
    }
}
