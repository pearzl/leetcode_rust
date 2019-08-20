// q0165_compare_version_numbers

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1: Vec<usize> = version1.split('.').map(|x| x.parse().unwrap()).collect();
        let mut v2: Vec<usize> = version2.split('.').map(|x| x.parse().unwrap()).collect();
        if v1.len() > v2.len() {
            for _ in 0..v1.len() - v2.len() {
                v2.push(0);
            }
        } else {
            for _ in 0..v2.len() - v1.len() {
                v1.push(0);
            }
        }
        if v1 == v2 {
            return 0;
        } else if v1 < v2 {
            return -1;
        } else {
            return 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            -1,
            Solution::compare_version(String::from("0.1"), String::from("1.1"))
        );
        assert_eq!(
            1,
            Solution::compare_version(String::from("1.0.1"), String::from("1"))
        );
        assert_eq!(
            -1,
            Solution::compare_version(String::from("7.5.2.4"), String::from("7.5.3"))
        );
        assert_eq!(
            0,
            Solution::compare_version(String::from("1.01"), String::from("1.001"))
        );
        assert_eq!(
            0,
            Solution::compare_version(String::from("1.0"), String::from("1.0.0"))
        );
    }
}
