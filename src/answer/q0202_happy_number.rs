// q0202_happy_number

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let base = |mut x| {
            let mut t = vec![];
            while x > 0 {
                t.push(x % 10);
                x /= 10;
            }
            t
        };
        let mut set = HashSet::new();
        set.insert(n);
        let mut new_n = n;
        loop {
            new_n = base(new_n).into_iter().map(|x| x * x).sum();
            if new_n == 1 {
                return true;
            }
            println!("{:?}--{}", set, new_n);
            if set.contains(&new_n) {
                return false;
            }
            set.insert(new_n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(true, Solution::is_happy(19));
    }
}
