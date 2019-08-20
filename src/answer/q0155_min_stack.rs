// q0155_min_stack

#[derive(Debug)]
struct MinStack {
    stack: Vec<i32>,
    sorted_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack: vec![],
            sorted_stack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        let index = match self.sorted_stack.binary_search(&x) {
            Ok(i) => i,
            Err(i) => i,
        };
        self.sorted_stack.insert(index, x);
    }

    fn pop(&mut self) {
        if let Some(x) = self.stack.pop() {
            let i = self.sorted_stack.binary_search(&x).unwrap();
            self.sorted_stack.remove(i);
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.sorted_stack[0]
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {
    // use super::Solution;

    use super::MinStack;

    fn run(actions: Vec<&str>, values: Vec<Vec<i32>>) -> Vec<String> {
        let mut ret = vec![];
        let mut ms = MinStack::new();
        for i in 0..actions.len() {
            if actions[i] == "MinStack" {
                ms = MinStack::new();
                ret.push(String::from("null"));
            } else if actions[i] == "push" {
                ms.push(values[i][0]);
                ret.push(String::from("null"));
            } else if actions[i] == "getMin" {
                let r = ms.get_min();
                ret.push(format!("{}", r));
            } else if actions[i] == "pop" {
                ms.pop();
                ret.push(String::from("null"));
            } else if actions[i] == "top" {
                let r = ms.top();
                ret.push(format!("{}", r));
            } else {
                panic!(1234567890)
            }
            println!("{}-{:?}->{:?}", actions[i], values[i], ms);
        }
        ret
    }

    fn output(otptstr: &str) -> Vec<String> {
        otptstr[1..otptstr.len() - 1]
            .split(',')
            .map(|s| format!("{}", s.trim()))
            .collect()
    }

    #[test]
    fn it_works() {
        assert_eq!(
            output("[null,null,null,null,-3,null,0,-2]"),
            run(
                vec!["MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin"],
                vec![
                    vec![],
                    vec![-2],
                    vec![0],
                    vec![-3],
                    vec![],
                    vec![],
                    vec![],
                    vec![]
                ]
            )
        );
    }
}
