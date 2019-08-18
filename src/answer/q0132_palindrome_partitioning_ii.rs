// q0132_palindrome_partitioning_ii

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        if Solution::is_plalindrome(&s) {
            return 0;
        }
        let ss = s.as_str();
        let mut times = 0;
        let mut start_indexs = HashSet::new();
        start_indexs.insert(0);
        while !start_indexs.contains(&ss.len()) {
            // println!("{:?}", start_indexs);
            let mut new_start_indexs = HashSet::new();
            for &start_i in start_indexs.iter() {
                for i in start_i..ss.len() {
                    if !Solution::is_plalindrome(&ss[start_i..=i]) {
                        continue;
                    }
                    if i == ss.len() - 1 {
                        return times;
                    }
                    new_start_indexs.insert(i + 1);
                }
            }
            std::mem::swap(&mut new_start_indexs, &mut start_indexs);
            times += 1;
        }
        times
    }

    fn is_plalindrome(s: &str) -> bool {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i..=i] != s[j..=j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::min_cut(String::from("aab")));
        assert_eq!(
            0,
            Solution::min_cut(String::from(
                "ababababababababababababcbabababababababababababa"
            ))
        );
        assert_eq!(
            1,
            Solution::min_cut(String::from(
                "ababababababababababababcbababababababababababab"
            ))
        );
        assert_eq!( 452, Solution::min_cut(String::from("apjesgpsxoeiokmqmfgvjslcjukbqxpsobyhjpbgdfruqdkeiszrlmtwgfxyfostpqczidfljwfbbrflkgdvtytbgqalguewnhvvmcgxboycffopmtmhtfizxkmeftcucxpobxmelmjtuzigsxnncxpaibgpuijwhankxbplpyejxmrrjgeoevqozwdtgospohznkoyzocjlracchjqnggbfeebmuvbicbvmpuleywrpzwsihivnrwtxcukwplgtobhgxukwrdlszfaiqxwjvrgxnsveedxseeyeykarqnjrtlaliyudpacctzizcftjlunlgnfwcqqxcqikocqffsjyurzwysfjmswvhbrmshjuzsgpwyubtfbnwajuvrfhlccvfwhxfqthkcwhatktymgxostjlztwdxritygbrbibdgkezvzajizxasjnrcjwzdfvdnwwqeyumkamhzoqhnqjfzwzbixclcxqrtniznemxeahfozp")));
    }
}
