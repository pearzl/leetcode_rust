// 22

fn main() {
    assert_eq!(
        vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string()
        ],
        generate_parenthesis(0)
    );
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut i = n;
    let mut o = 0;
    let mut ret = vec![];
    let mut st = vec![(n, 0, String::new())];
    while st[0].0 > 0 || st[0].1 > 0 {
        st = state(st);
    }
    for item in st.iter() {
        ret.push(item.2.clone())
    }
    ret
}

fn state(v: Vec<(i32, i32, String)>) -> Vec<(i32, i32, String)> {
    let mut ret = vec![];
    for item in v.iter() {
        if item.0 > 0 {
            let mut s = item.2.clone();
            s.push_str("(");
            ret.push((item.0 - 1, item.1 + 1, s));
        }
        if item.1 > 0 {
            let mut s = item.2.clone();
            s.push_str(")");
            ret.push((item.0, item.1 - 1, s));
        }
    }
    ret
}
