// q0065_valid_number

struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let input = s.trim();
        if input.len() == 0 {
            return false;
        }
        let mut base_exp = 0;
        let mut base = String::new();
        let mut state = 0;
        for &c in input.as_bytes() {
            if base_exp == 0 {
                //base
                if state == 0 {
                    // start
                    if c == b'+' || c == b'-' {
                        state = 1;
                    } else if c >= b'0' && c <= b'9' {
                        state = 2;
                        base.push(c as char)
                    } else if c == b'.' {
                        state = 3;
                        base.push(c as char)
                    } else {
                        return false;
                    }
                } else if state == 1 {
                    // flag +/-
                    if c >= b'0' && c <= b'9' {
                        state = 2;
                        base.push(c as char)
                    } else if c == b'.' {
                        state = 3;
                        base.push(c as char)
                    } else {
                        return false;
                    }
                } else if state == 2 {
                    // integers
                    if c >= b'0' && c <= b'9' {
                        base.push(c as char)
                    } else if c == b'.' {
                        state = 3;
                        base.push(c as char)
                    } else if c == b'e' || c == b'E' {
                        if base == "." {
                            return false;
                        } else {
                            state = 0;
                            base_exp = 1;
                        }
                    } else {
                        return false;
                    }
                } else if state == 3 {
                    // radix point
                    if c >= b'0' && c <= b'9' {
                        state = 4;
                        base.push(c as char)
                    } else if c == b'e' || c == b'E' {
                        if base == "." {
                            return false;
                        } else {
                            state = 0;
                            base_exp = 1;
                        }
                    } else {
                        return false;
                    }
                } else if state == 4 {
                    // decimals
                    if c >= b'0' && c <= b'9' {
                        base.push(c as char)
                    } else if c == b'e' || c == b'E' {
                        if base == "." {
                            return false;
                        } else {
                            state = 0;
                            base_exp = 1;
                        }
                    } else {
                        return false;
                    }
                } else {
                    panic!("1");
                }
            } else {
                if state == 0 {
                    // start
                    if c == b'+' || c == b'-' {
                        state = 1;
                    } else if c >= b'0' && c <= b'9' {
                        state = 2;
                    } else {
                        return false;
                    }
                } else if state == 1 {
                    // flag
                    if c >= b'0' && c <= b'9' {
                        state = 2;
                    } else {
                        return false;
                    }
                } else if state == 2 {
                    // number
                    if !(c >= b'0' && c <= b'9') {
                        return false;
                    }
                } else {
                    panic!(2);
                }
            }
        }
        if base_exp == 1 && state == 2 {
            return true;
        } else if base_exp == 0 {
            if base != "." {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_number(String::from("0")), true);
        assert_eq!(Solution::is_number(String::from(" 0.1 ")), true);
        assert_eq!(Solution::is_number(String::from("abc")), false);
        assert_eq!(Solution::is_number(String::from("1 a")), false);
        assert_eq!(Solution::is_number(String::from("2e10")), true);
        assert_eq!(Solution::is_number(String::from(" -90e3   ")), true);
        assert_eq!(Solution::is_number(String::from(" 1e")), false);
        assert_eq!(Solution::is_number(String::from("e3")), false);
        assert_eq!(Solution::is_number(String::from(" 6e-1")), true);
        assert_eq!(Solution::is_number(String::from(" 99e2.5 ")), false);
        assert_eq!(Solution::is_number(String::from("53.5e93")), true);
        assert_eq!(Solution::is_number(String::from(" --6 ")), false);
        assert_eq!(Solution::is_number(String::from("-+3")), false);
        assert_eq!(Solution::is_number(String::from("95a54e53")), false);
        assert_eq!(Solution::is_number(String::from(".1")), true);
        assert_eq!(Solution::is_number(String::from("3.")), true);
        assert_eq!(Solution::is_number(String::from(" .")), false);
        assert_eq!(Solution::is_number(String::from("+.8")), true);
        assert_eq!(Solution::is_number(String::from("46.e3")), true);
        assert_eq!(Solution::is_number(String::from(".e3")), false);
        assert_eq!(Solution::is_number(String::from("-.1e3")), true);
        assert_eq!(Solution::is_number(String::from("..2")), false);
    }
}
