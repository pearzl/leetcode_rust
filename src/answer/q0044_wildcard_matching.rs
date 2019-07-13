// q0044_wildcard_matching 


struct Solution;

// impl Solution {
//     pub fn is_match(s: String, p: String) -> bool {
//         if p == "" {
//             if s == "" {
//                 return true;
//             }else {
//                 return false;
//             }
//         }
//         let pp = p.replace('*', "_*_").replace('?', "_?_").replace("__", "_");
//         let patterns: Vec<&str> = pp.trim_matches('_').split('_').collect();
//         // println!("pattern {:?}", patterns);

//         Solution::match_patterns(&s, &patterns[..])
//     }

//     fn match_patterns(s: &str, patterns: &[&str]) -> bool {
//         // println!("matching : {}--{:?}", s, patterns);
//         if patterns.len() == 0 && s != "" {
//             return false;
//         }
//         if s == "" {
//             for c in patterns {
//                 if *c != "*" {
//                     return false;
//                 }
//             }
//             return true
//         }
//         let pat = &patterns[0];
//         let pos_plc = Solution::smatch_one_pattern(pat, s);
//         if pos_plc.len() == 0 {
//             return false;
//         }else {
//             for plc in pos_plc.iter() {
//                 if Solution::match_patterns(&s[*plc..], &patterns[1..]) {
//                     return true;
//                 }
//             }
//             false
//         }
        
//     }

//     fn smatch_one_pattern(pattern: &str, s: &str) -> Vec<usize> {
//         let mut ret = vec![];
//         if pattern == "*" {
//             ret = (0..=s.len()).map(|i| i).collect();
//         }else if pattern == "?" {
//             if s.len() != 0 {
//                 ret.push(1);
//             }
//         }else {
//             if s.starts_with(pattern) {
//                 ret.push(pattern.len());
//             }
//         }
//         ret
//     }
// }

use std::collections::HashSet;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p == "" {
            if s == "" {
                return true;
            }else {
                return false;
            }
        }

        let mut p = p;
        loop{
            let tp = p.replace("**", "*");
            if tp == p {
                break;
            }else {
                p = tp;
            }
        }

        let patterns: Vec<char> = p.chars().collect();
        println!("pattern {:?}", patterns);

        let mut posp = vec![0];
        for c in s.chars() {
            println!("matching {} in {:?} from {:?}", c, patterns, posp);
            Solution::match_pos(c, &patterns, &mut posp);
            if posp.len() == 0 {
                return false;
            }
        }
        println!("exit with {:?}", posp);
        // for pos in posp.iter() {
        //     if *pos < s.len() {
        //         for (j, pat) in patterns.iter().enumerate() {
        //             if j < *pos {
        //                 continue
        //             }
        //             if *pat != '*' {
        //                 return false
        //             }
        //         }
        //     }
        // }
        let li = posp.iter().max().unwrap();
        println!("max index: {}", li);
        for i in *li..patterns.len() {
            println!("pattern: {}--{}", i, patterns[i]);
            if patterns[i] != '*' {
                return false;
            }
        }
        println!("return here");
        true
    }

    // fn match_patterns(s: String ,)

    fn match_pos(c: char, patterns: &Vec<char>, from: &mut Vec<usize>){
        let mut ret = HashSet::new();
        for ofst in from.iter() {
            if *ofst >= patterns.len() {
                continue;
            }
            let p = patterns[*ofst];
            if p == '*' {
                ret.insert(*ofst);
                if ofst + 1 < patterns.len() {
                    let np = patterns[ofst+1];
                    if np == '?' {
                        ret.insert(ofst+2);
                    }else if np == c {
                        ret.insert(ofst+2);
                    }
                }
            }else if p == '?' {
                ret.insert(ofst+1);
            }else if p == c {
                ret.insert(ofst+1);
            }           
        }
        let r: Vec<usize> = ret.iter().cloned().collect();
        std::mem::replace(from, r);
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!( true, Solution::is_match(String::from(""), String::from("******")) );
        assert_eq!( true, Solution::is_match(String::from(""), String::from("")) );
        assert_eq!( false, Solution::is_match(String::from("1"), String::from("")) );
        assert_eq!( false, Solution::is_match(String::from("aa"), String::from("a")) );
        assert_eq!( true, Solution::is_match(String::from("aa"), String::from("a*")) );
        assert_eq!( false, Solution::is_match(String::from("cb"), String::from("?a")) );
        assert_eq!( true, Solution::is_match(String::from("adceb"), String::from("*a*b")) );
        assert_eq!( true, Solution::is_match(String::from("acdcb"), String::from("acdcb*******")) );
        assert_eq!( false, Solution::is_match(String::from("acdcb"), String::from("**a*c?b")) );
        assert_eq!( false, Solution::is_match(String::from("aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba"), String::from("a*******b")) );
        assert_eq!( false, Solution::is_match(String::from("mississippi"), String::from("m??*ss*?i*pi")) );
    }
}

