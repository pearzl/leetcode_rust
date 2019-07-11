// q0020_valid_parentheses

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = vec![];
        for &c in s.as_bytes() {
            if c == b'{' || c == b'[' || c == b'(' {
                v.push(c);
            } else if c == b'}' {
                match v.pop() {
                    Some(f) => {
                        if f != b'{' {
                            return false;
                        }
                    }
                    None => return false,
                }
            } else if c == b']' {
                match v.pop() {
                    Some(f) => {
                        if f != b'[' {
                            return false;
                        }
                    }
                    None => return false,
                }
            } else if c == b')' {
                match v.pop() {
                    Some(f) => {
                        if f != b'(' {
                            return false;
                        }
                    }
                    None => return false,
                }
            } else {
                panic!("invalid input");
            }
        }
        if v.len() > 0 {
            return false;
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }
}
