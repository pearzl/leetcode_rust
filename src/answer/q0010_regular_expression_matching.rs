// q0010_regular_expression_matching

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut mode = vec![];
        let ps = p.as_bytes();
        let ss = s.as_bytes();
        let mut i = 0;
        while i < ps.len() {
            let c = ps[i];
            if c != b'.' && (c > b'z' || c < b'a') {
                return false;
            }
            if i != ps.len() - 1 && ps[i + 1] == b'*' {
                i += 1;
                mode.push((c, true))
            } else {
                mode.push((c, false))
            }
            i += 1;
        }
        println!("mode: {:?}", mode);

        let mut posb = vec![0];
        for c in ss {
            posb = smatch(*c, posb, &mode);
            // println!("matching: {} on {:?}", *c as char, posb);
            if posb.len() == 0 {
                return false;
            }
        }
        if !posb.contains(&mode.len()) {
            for i in posb.iter() {
                let mut n = *i;
                let mut allm = true;
                while n < mode.len() {
                    let (_, f) = mode[n];
                    if !f {
                        allm = false;
                        break;
                    }
                    n += 1;
                }
                if allm {
                    return true;
                }
            }
            return false;
        }
        true
    }
}

// 从m的p[i]个位置开始匹配字符c，返回下一个字符开始匹配的位置
fn smatch(c: u8, p: Vec<usize>, m: &Vec<(u8, bool)>) -> Vec<usize> {
    let mut ret = vec![];
    println!("smatch: {},{:?},{:?}", c as char, p, ret);
    for i in p.iter() {
        if let Some((v, f)) = m.get(*i) {
            if !f {
                if *v == b'.' || *v == c {
                    let t = *i + 1;
                    println!("!f: {}", t);
                    if t <= m.len() && !ret.contains(&t) {
                        ret.push(t);
                    }
                }
            } else {
                if *v == b'.' || *v == c {
                    let t = *i + 1;
                    if t <= m.len() && !ret.contains(&t) {
                        ret.push(t);
                    }
                    let t = *i;
                    if t <= m.len() && !ret.contains(&t) {
                        ret.push(t);
                    }
                }
                for i in smatch(c, vec![*i + 1], m).iter() {
                    let t = *i;
                    if t <= m.len() && !ret.contains(&t) {
                        ret.push(t);
                    }
                    // println!("{}--{:?}",i, ret);
                }
            }
        }
    }
    // println!("smatch1: {},{:?},{:?}", c as char , p,  ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::is_match(
                "aaaaaaaaaaaaab".to_string(),
                "a*a*a*a*a*a*a*a*a*a*a*a*b".to_string()
            ),
            true
        );
        assert_eq!(
            Solution::is_match("ab".to_string(), ".*..c*".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("bbbba".to_string(), ".*a*a".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("aaa".to_string(), "a*a".to_string()),
            true
        );
        assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(
            Solution::is_match("aab".to_string(), "c*a*b".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
            false
        );
    }
}
