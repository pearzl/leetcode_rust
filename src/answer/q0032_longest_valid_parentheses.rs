// q0032_longest_valid_parentheses

struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let ss = s.as_bytes();
        let slen = ss.len();
        if slen < 2 {
            return 0;
        }
        let mut valid_pos = vec![];
        let mut i = 0;
        while i < slen - 1 {
            if ss[i] == b'(' && ss[i + 1] == b')' {
                valid_pos.push((i, i + 1));
                i += 1;
            }
            i += 1
        }
        loop {
            println!("round vp: {:?}", valid_pos);
            let mut no_change = true;
            let mut pvpe = 0; //previous valid postion end index
            let mut new_valid_pos = vec![];
            let mut i = 0;
            while i < valid_pos.len() && pvpe < slen {
                if i == valid_pos.len() - 1 {
                    let vp = valid_pos[i];
                    if (pvpe == 0 || vp.0 > pvpe + 1) && vp.1 + 1 < slen && vp.0 != 0 {
                        if ss[vp.0 - 1] == b'(' && ss[vp.1 + 1] == b')' {
                            new_valid_pos.push((vp.0 - 1, vp.1 + 1));
                            no_change = false;
                            pvpe = vp.1 + 1;
                        } else {
                            new_valid_pos.push((vp.0, vp.1));
                            pvpe = vp.1;
                        }
                    } else {
                        new_valid_pos.push((vp.0, vp.1));
                        pvpe = vp.1;
                    }
                } else {
                    let vp1 = valid_pos[i];
                    let vp2 = valid_pos[i + 1];
                    if (pvpe == 0 || pvpe + 1 < vp1.0) && vp1.1 + 1 < vp2.0 && vp1.0 != 0 {
                        if ss[vp1.0 - 1] == b'(' && ss[vp1.1 + 1] == b')' {
                            new_valid_pos.push((vp1.0 - 1, vp1.1 + 1));
                            no_change = false;
                            pvpe = vp1.1 + 1;
                        } else {
                            new_valid_pos.push((vp1.0, vp1.1));
                            pvpe = vp1.1;
                        }
                    } else if vp1.1 + 1 == vp2.0 {
                        new_valid_pos.push((vp1.0, vp2.1));
                        no_change = false;
                        pvpe = vp2.1;
                        i += 1;
                    } else {
                        new_valid_pos.push((vp1.0, vp1.1));
                        pvpe = vp1.1;
                    }
                }
                i += 1;
            }
            if no_change {
                break;
            }
            std::mem::swap(&mut new_valid_pos, &mut valid_pos);
        }
        println!("final: {:?}", valid_pos);
        let mut max_len = 0;
        for (a, b) in valid_pos.iter() {
            max_len = max_len.max(b - a + 1);
        }
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!( 4, Solution::longest_valid_parentheses(String::from(")()())")));
        // assert_eq!( 2, Solution::longest_valid_parentheses(String::from("(()")));
        // assert_eq!( 6, Solution::longest_valid_parentheses(String::from("()()()")));
        // assert_eq!( 6, Solution::longest_valid_parentheses(String::from("()(())")));
        // assert_eq!( 6, Solution::longest_valid_parentheses(String::from("(()())")));
        assert_eq!(
            8,
            Solution::longest_valid_parentheses(String::from("((()))())"))
        );
    }
}
