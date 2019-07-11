// q0014_longest_common_prefix

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        } else if strs.len() == 1 {
            return strs[0].to_string();
        }
        let mut ret = String::new();
        let goal = &strs[0];
        let mut i = 0;
        loop {
            if i >= goal.as_str().len() {
                break;
            }
            let mut exit = false;
            for cs in &strs[1..] {
                // println!("{}", cs);
                if i >= cs.as_str().len() {
                    exit = true;
                    break;
                }
                if goal.as_str()[i..=i] != cs.as_str()[i..=i] {
                    exit = true;
                    break;
                }
            }
            if exit {
                break;
            }
            i += 1;
        }
        goal[0..i].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            "fl".to_string(),
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ])
        );
        assert_eq!("".to_string(), Solution::longest_common_prefix(vec![]));
        assert_eq!(
            "".to_string(),
            Solution::longest_common_prefix(vec!["".to_string(), "b".to_string()])
        );
    }
}
