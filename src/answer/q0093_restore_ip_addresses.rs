// q0093_restore_ip_addresses

struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ret = vec![];
        let l1 = s.len();
        if 4 <= l1 && l1 <= 12 {
            // num1
            // println!("search 1 num");
            for i1 in 0..3 {
                let n1 = &s[0..=i1];
                // println!("  try n1:{}", n1);
                if Solution::valid_num(n1) {
                    let l2 = l1 - i1 - 1;
                    if 3 <= l2 && l2 <= 9 {
                        // num2
                        // println!("  search 2 num");
                        for i2 in i1 + 1..i1 + 4 {
                            if i2 >= l1 {
                                // println!("  i2 beyond: {}", i2);
                                break;
                            }
                            let n2 = &s[i1 + 1..=i2];
                            // println!("    try n2:{}", n2);
                            if Solution::valid_num(n2) {
                                let l3 = l1 - i2 - 1;
                                if 2 <= l3 && l3 <= 6 {
                                    //num3
                                    // println!("    search 3 num");
                                    for i3 in i2 + 1..i2 + 4 {
                                        if i3 >= l1 {
                                            // println!("    i3 beyond: {}", i3);
                                            break;
                                        }
                                        let n3 = &s[i2 + 1..=i3];
                                        // println!("      try n3:{}", n3);
                                        if Solution::valid_num(n3) {
                                            let l4 = l1 - i3 - 1;
                                            if 1 <= l4 && l4 <= 3 {
                                                // num4
                                                let n4 = &s[i3 + 1..];
                                                // println!("      determine: {}-{}-{}-{}", n1, n2, n3, n4);
                                                if Solution::valid_num(n4) {
                                                    let mut ip = String::with_capacity(l1);
                                                    ip.push_str(n1);
                                                    ip.push('.');
                                                    ip.push_str(n2);
                                                    ip.push('.');
                                                    ip.push_str(n3);
                                                    ip.push('.');
                                                    ip.push_str(n4);
                                                    ret.push(ip);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        ret
    }

    pub fn valid_num(s: &str) -> bool {
        let slen = s.len();
        assert!(slen > 0 && slen < 4);
        if slen == 1 {
            return true;
        } else if slen == 2 {
            return &s[0..=0] != "0";
        } else {
            if &s[0..=0] == "1" {
                return true;
            } else if &s[0..=0] == "2" {
                if ["0", "1", "2", "3", "4"].contains(&&s[1..=1]) {
                    return true;
                } else if &s[1..=1] == "5" {
                    return ["0", "1", "2", "3", "4", "5"].contains(&&s[2..=2]);
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util;

    #[test]
    fn it_works() {
        assert_eq!(
            util::vec_2_set(vec![
                String::from("255.255.11.135"),
                String::from("255.255.111.35")
            ]),
            util::vec_2_set(Solution::restore_ip_addresses(String::from("25525511135")))
        );
        assert_eq!(
            util::vec_2_set(vec![String::from("0.10.0.0")]),
            util::vec_2_set(Solution::restore_ip_addresses(String::from("01000")))
        );
        assert_eq!(
            util::vec_2_set(vec![String::from("0.0.0.0")]),
            util::vec_2_set(Solution::restore_ip_addresses(String::from("0000")))
        );
        assert_eq!(
            util::vec_2_set(vec![String::from("0.0.1.0")]),
            util::vec_2_set(Solution::restore_ip_addresses(String::from("0010")))
        );
        assert_eq!(
            util::vec_2_set(Vec::<String>::new()),
            util::vec_2_set(Solution::restore_ip_addresses(String::from("")))
        );
    }

    #[test]
    fn is_ip_num() {
        assert_eq!(true, Solution::valid_num("1"));
        assert_eq!(true, Solution::valid_num("10"));
        assert_eq!(true, Solution::valid_num("100"));
        assert_eq!(true, Solution::valid_num("200"));
        assert_eq!(true, Solution::valid_num("240"));
        assert_eq!(true, Solution::valid_num("250"));
        assert_eq!(true, Solution::valid_num("255"));
        assert_eq!(true, Solution::valid_num("249"));
        assert_eq!(true, Solution::valid_num("244"));
        assert_eq!(true, Solution::valid_num("255"));
        assert_eq!(true, Solution::valid_num("0"));
        assert_eq!(false, Solution::valid_num("01"));
        assert_eq!(false, Solution::valid_num("256"));
        assert_eq!(false, Solution::valid_num("060"));
    }
}
