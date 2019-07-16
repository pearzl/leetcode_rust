// q0071_simplify_path

struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut tmp: Vec<&str> = path.split('/').collect();
        let mut tmp1: Vec<&str> = vec![];
        let mut back = 0;
        while let Some(p) = tmp.pop() {
            if p == "" {
                continue;
            } else if p == "." {
                continue;
            } else if p == ".." {
                back += 1;
            } else {
                if back == 0 {
                    tmp1.push(p);
                } else {
                    back -= 1;
                }
            }
        }
        let mut ret = String::from("/");
        for ss in tmp1.iter().rev() {
            ret.push_str(ss);
            ret.push('/')
        }
        if ret.len() > 1 {
            ret.pop();
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::simplify_path(String::from("/../")),
            String::from("/")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home/abc")),
            String::from("/home/abc")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home/abc/")),
            String::from("/home/abc")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home//abc")),
            String::from("/home/abc")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home/abc/./")),
            String::from("/home/abc")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home/a/../abc")),
            String::from("/home/abc")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home/././abc")),
            String::from("/home/abc")
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home//abc///")),
            String::from("/home/abc")
        );
    }
}
