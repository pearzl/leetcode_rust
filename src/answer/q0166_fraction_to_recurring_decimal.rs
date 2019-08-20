// q0166_fraction_to_recurring_decimal

struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if denominator == 0 {
            return String::from("NaN");
        }
        let mut numerator = numerator as i64;
        let mut denominator = denominator as i64;
        let flag_pos = if numerator < 0 && denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
            true
        } else if numerator < 0 {
            numerator = -numerator;
            false
        } else if denominator < 0 {
            denominator = -denominator;
            false
        } else {
            true
        };
        let intergal = numerator / denominator;
        let mut remainder = numerator % denominator;
        if remainder == 0 {
            if flag_pos {
                return format!("{}", intergal);
            } else {
                if intergal == 0 {
                    return format!("0");
                }
                return format!("-{}", intergal);
            }
        }
        let mut remainders = vec![];
        let mut fractional = vec![];
        let search = |v: &Vec<i64>, x: i64| {
            let mut r = None;
            for i in 0..v.len() {
                if v[i] == x {
                    r = Some(i);
                    break;
                }
            }
            r
        };
        while remainder != 0 {
            remainders.push(remainder);
            let n = remainder * 10;
            fractional.push((n / denominator) as u8);
            remainder = n % denominator;
            if remainder == 0 {
                break;
            }
            if let Some(index) = search(&remainders, remainder) {
                let mut s = if flag_pos {
                    format!("{}.", intergal)
                } else {
                    format!("-{}.", intergal)
                };
                for i in fractional[0..index].into_iter() {
                    s.push((i + b'0') as char);
                }
                s.push('(');
                for i in fractional[index..].into_iter() {
                    s.push((i + b'0') as char);
                }
                s.push(')');
                return s;
            }
        }
        let mut s = if flag_pos {
            format!("{}.", intergal)
        } else {
            format!("-{}.", intergal)
        };
        for i in fractional.into_iter() {
            s.push((i + b'0') as char);
        }
        return s;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(String::from("0.5"), Solution::fraction_to_decimal(1, 2));
        assert_eq!(String::from("2"), Solution::fraction_to_decimal(2, 1));
        assert_eq!(String::from("0.(6)"), Solution::fraction_to_decimal(2, 3));
        assert_eq!(
            String::from("27.(27)"),
            Solution::fraction_to_decimal(2700, 99)
        );
        assert_eq!(String::from("0.1(6)"), Solution::fraction_to_decimal(1, 6));
        assert_eq!(
            String::from("-0.58(3)"),
            Solution::fraction_to_decimal(7, -12)
        );
        assert_eq!(
            String::from("0.0000000004656612873077392578125"),
            Solution::fraction_to_decimal(-1, -2147483648)
        );
        assert_eq!(
            String::from("-2147483648"),
            Solution::fraction_to_decimal(-2147483648, 1)
        );
        assert_eq!(
            String::from("2147483648"),
            Solution::fraction_to_decimal(-2147483648, -1)
        );
        assert_eq!(String::from("0"), Solution::fraction_to_decimal(0, -1));
    }
}
