// q0115_distinct_subsequences

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut ns = String::new();
        for c in s.chars() {
            if t.contains(c) {
                ns.push(c)
            }
        }
        Solution::nd(ns, t)
    }

    fn nd(s: String, t: String) -> i32 {
        if t.len() == 0 {
            return 1;
        }
        if s.len() == 0 {
            return 0;
        }
        let t_byte = t.as_bytes();
        let mut buf = vec![vec![0; t.len()]; s.len()]; // 在s的前i个位置组成t的前j个字符的组合的数目
        let mut count_first = 0;
        let firtst_tchar = t_byte[0];
        for (i, bs) in s.bytes().enumerate() {
            if firtst_tchar == bs {
                count_first += 1;
            }
            buf[i][0] = count_first;
        }
        // println!("init: {:?}", buf);
        for j in 1..t.len() {
            for i in 0..s.len() {
                if i < j {
                    continue;
                }
                for (index, sb) in s.bytes().take(i + 1).enumerate() {
                    // print!("{}", sb as char);
                    if sb == t_byte[j] {
                        if index != 0 {
                            buf[i][j] += buf[index - 1][j - 1];
                        }
                    }
                }
                // println!("{} , search {} in {}", buf[i][j], &t[0..=j], &s[0..=i]);
            }
        }
        // println!("{:?}", buf);
        buf[s.len() - 1][t.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::num_distinct(String::from("rabbbit"), String::from("rabbit"))
        );
        assert_eq!(
            5,
            Solution::num_distinct(String::from("babgbag"), String::from("bag"))
        );
        assert_eq!(
            3,
            Solution::num_distinct(String::from("ddd"), String::from("dd"))
        );
        assert_eq!(
            1,
            Solution::num_distinct(String::from("babgbag"), String::from(""))
        );
        assert_eq!(
            1,
            Solution::num_distinct(
                String::from(
                    "alksdgjqwoeitjaodgmjoqweriotjlqpoewithjeoibvnqeuhgiouasbfuidkdfajsfbiuebvfiu"
                ),
                String::from("akl")
            )
        );
        assert_eq!(
            23,
            Solution::num_distinct(
                String::from(
                    "alksdgjqwoeitjaodgmjoqweriotjlqpoewithjeoibvnqeuhgiouasbfuidkdfajsfbiuebvfiu"
                ),
                String::from("aqo")
            )
        );
        assert_eq!( 700531452, Solution::num_distinct(String::from("adbdadeecadeadeccaeaabdabdbcdabddddabcaaadbabaaedeeddeaeebcdeabcaaaeeaeeabcddcebddebeebedaecccbdcbcedbdaeaedcdebeecdaaedaacadbdccabddaddacdddc"), String::from("bcddceeeebecbc")));
    }
}
