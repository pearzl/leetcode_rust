// q0076_minimum_window_substring

struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s == "" || t == "" {
            return String::new();
        }
        let ss: Vec<(usize, char)> = s
            .chars()
            .enumerate()
            .filter(|(_i, c)| t.contains(c.to_string().as_str()))
            .collect();
        println!("{:?}", ss);
        if ss.len() < t.len() {
            return String::new();
        }
        let mut char_need: HashMap<char, isize> = HashMap::new();
        for c in t.chars() {
            if let Some(n) = char_need.get_mut(&c) {
                *n += 1;
            } else {
                char_need.insert(c, 1);
            }
        }
        let mut lb = 0;
        let mut rb = 0;
        let mut min_range = None;
        let mut right_move = true;
        loop {
            if right_move {
                println!("right: {}-{}--{:?}", lb, rb, char_need);
                if Solution::contains(&char_need) {
                    // from lb to rb-1
                    right_move = false;
                } else {
                    if rb == ss.len() {
                        break;
                    }
                    let n = char_need.get_mut(&ss[rb].1).unwrap();
                    *n -= 1;
                    rb += 1;
                }
            } else {
                println!("left: {}-{}--{:?}", lb, rb, char_need);
                let n = char_need.get_mut(&ss[lb].1).unwrap();
                *n += 1;
                if Solution::contains(&char_need) {
                    // from lb+1 to rb-1
                    lb += 1;
                } else {
                    println!("find range: {}--{}", lb, rb - 1);
                    let cur_range = (ss[lb].0, ss[rb - 1].0);
                    match min_range {
                        None => min_range = Some(cur_range),
                        Some(mr) => {
                            if cur_range.1 - cur_range.0 < mr.1 - mr.0 {
                                min_range = Some(cur_range)
                            }
                        }
                    }
                    lb += 1;
                    right_move = true;
                }
            }
        }
        match min_range {
            None => return String::new(),
            Some(r) => return String::from(&s[r.0..=r.1]),
        }
    }

    fn contains(map: &HashMap<char, isize>) -> bool {
        for (_k, v) in map.iter() {
            if *v > 0 {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::min_window(String::from("BbabBbbaab"), String::from("Baa")),
            String::from("Bbbaa")
        );
        assert_eq!(
            Solution::min_window(String::from("ADOBECODEBANC"), String::from("ABC")),
            String::from("BANC")
        );
        assert_eq!(
            Solution::min_window(String::from("ADOBECODEBANC"), String::from("")),
            String::from("")
        );
        assert_eq!(
            Solution::min_window(String::from("ADOBECODEBANC"), String::from("A")),
            String::from("A")
        );
        assert_eq!(
            Solution::min_window(String::from("ADO"), String::from("ABCDEDF")),
            String::from("")
        );
        assert_eq!( Solution::min_window(String::from("ADOASDFKWEEIODFGJWERIWEQOKAFSDJQWETOWERQDAFGRKWQEJRISADFJSAGIKWEIGJSEFJOQWERIQWETGJQWORQWOFGKJEGJASDIRTOIWEQGEKG"), String::from("ADFGASGVAEERGEQRR")), String::from(""));
        assert_eq!( Solution::min_window(String::from("ADOAS>DFKWEEIPWOERIQWTKGEHDFBLDFKGKJSADFGJSDHGHSNGQERHGREQTIUYREUYERQUETORPOQWREPOADFKVFKMVXCZMMHFGLFHGJGLFJMNHPPRTEIRYTWIRTYRTJFGJHBDFGOERWTOERITRHJHBFDKMODFGJWERIWEQOKAFSDJQWETOWERQDAFGRKWQEJRISADFJSAGIKWEIGJSEFJOQWERIQWETGJQWORQWOFGKJEGJASDIRTOIWEQGEKG"), String::from("ADFGASGVAEERGEQRR")), String::from("AS>DFKWEEIPWOERIQWTKGEHDFBLDFKGKJSADFGJSDHGHSNGQERHGREQTIUYREUYERQUETORPOQWREPOADFKV"));
    }

}
