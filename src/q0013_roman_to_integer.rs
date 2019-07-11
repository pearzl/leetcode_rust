// q0013_roman_to_integer

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let c = [
            "I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M",
        ];
        let n = [1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000];
        let mut i = n.len() - 1;
        let mut ret = 0;
        let mut index = 0;
        let mut sp = &s[..];
        while sp.len() > 0 {
            if sp.starts_with(c[i]) {
                ret += n[i];
                index += c[i].len();
                sp = &s[index..];
            } else {
                i -= 1;
            }
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(5, Solution::roman_to_int("V".to_string()));
        assert_eq!(3, Solution::roman_to_int("III".to_string()));
        assert_eq!(4, Solution::roman_to_int("IV".to_string()));
        assert_eq!(9, Solution::roman_to_int("IX".to_string()));
        assert_eq!(58, Solution::roman_to_int("LVIII".to_string()));
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_string()));
    }
}
