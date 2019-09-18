// q0214_shortest_palindrome 


struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let ss = s.as_bytes();
        let len = ss.len();
        let mut append_from = 0;
        for i in (0..len).rev() {
            let (mut p1, mut p2) = if i % 2 == 1 {
                let t = i / 2;
                (t, t + 1)
            }else {
                let t = i / 2;
                (t, t)
            };
            
            let mut is_palindrome = true;
            loop {
                if ss[p1] != ss[p2] {
                    is_palindrome = false;
                    break;
                }
                if p1 == 0 {
                    break
                }else {
                    p1 -= 1;
                    p2 += 1;
                }
            }

            if is_palindrome {
                append_from = p2 + 1;
                break
            }
        }
        let mut ret = Vec::with_capacity(2*len-append_from);
        for c in ss[append_from..].iter().rev() {
            ret.push(*c);
        }
        ret.extend_from_slice(ss);
        String::from_utf8(ret).unwrap()
    }
}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!( String::from("abcba"), Solution::shortest_palindrome(String::from("cba")));
        assert_eq!( String::from("abcba"), Solution::shortest_palindrome(String::from("bcba")));
        assert_eq!( String::from("abcba"), Solution::shortest_palindrome(String::from("abcba")));
        assert_eq!( String::from(""), Solution::shortest_palindrome(String::from("")));
        assert_eq!( String::from("abccba"), Solution::shortest_palindrome(String::from("ccba")));
        assert_eq!( String::from("abccba"), Solution::shortest_palindrome(String::from("ccba")));
        assert_eq!( String::from("abccba"), Solution::shortest_palindrome(String::from("bccba")));
        assert_eq!( String::from("abccba"), Solution::shortest_palindrome(String::from("bccba")));
        assert_eq!( String::from("abccba"), Solution::shortest_palindrome(String::from("abccba")));
        assert_eq!( String::from("xyzyxbabxyzyx"), Solution::shortest_palindrome(String::from("abxyzyx")));
        assert_eq!( String::from("baxyzyxab"), Solution::shortest_palindrome(String::from("xyzyxab")));
    }
}

