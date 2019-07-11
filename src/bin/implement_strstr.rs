// 28

pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(needle.as_str()){
        Some(n) => n as i32,
        None => -1
    }
}

fn main() {
    assert_eq!(2, str_str("hello".to_string(), "ll".to_string()));
    assert_eq!(-1, str_str("aaaaa".to_string(), "bba".to_string()));
}