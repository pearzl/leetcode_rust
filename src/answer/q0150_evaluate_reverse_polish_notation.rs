// q0150_evaluate_reverse_polish_notation

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut ret: Vec<i32> = vec![];
        for item in tokens.into_iter() {
            print!("{}", item);
            if item == "+" {
                let n1 = ret.pop().unwrap();
                let n2 = ret.pop().unwrap();
                ret.push(n1 + n2);
            } else if item == "-" {
                let n1 = ret.pop().unwrap();
                let n2 = ret.pop().unwrap();
                ret.push(n2 - n1);
            } else if item == "*" {
                let n1 = ret.pop().unwrap();
                let n2 = ret.pop().unwrap();
                ret.push(n1 * n2);
            } else if item == "/" {
                let n1 = ret.pop().unwrap();
                let n2 = ret.pop().unwrap();
                ret.push(n2 / n1);
            } else {
                let n = item.parse::<i32>().unwrap();
                ret.push(n);
            }
        }
        ret.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            6,
            Solution::eval_rpn(vec![
                String::from("4"),
                String::from("13"),
                String::from("5"),
                String::from("/"),
                String::from("+")
            ])
        );
    }
}
