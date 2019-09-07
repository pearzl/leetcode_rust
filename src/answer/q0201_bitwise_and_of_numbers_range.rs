// q0201_bitwise_and_of_numbers_range

struct Solution;

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut base = 1;
        let mut ret = 0;
        for i in 0..32 {
            let mut all_one = true;
            for x in m >> i..=n >> i {
                if x % 2 == 0 {
                    all_one = false;
                    break;
                }
            }
            if all_one {
                ret += base;
            }
            base <<= 1;
            if base > n {
                break;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));
        assert_eq!(0, Solution::range_bitwise_and(0, 1));
        assert_eq!(6, Solution::range_bitwise_and(6, 7));
        assert_eq!(0, Solution::range_bitwise_and(0, 0));
    }
}
