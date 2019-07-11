// generate init TEMPLATE source;
// modify lib file.
// usage: cargo run <title> <num>

const TEMPLATE: &'static str = r###"

struct Solution;

impl Solution {
    
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!( , );
    }
}

"###;

use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    args.next();
    let title = args.next().unwrap().replace("-", "_");
    let num: usize = args.next().unwrap().parse().unwrap();
    // let title = format!("q{:04}_{} ", num, title.trim_end_matches(|c: char| !(c.is_ascii_alphanumeric() || c == '_')));
    let title = format!("q{:04}_{}", num, title.trim());

    fs::write(
        format!("src/{}.rs", title),
        format!("// {} \n{}", title, TEMPLATE).as_bytes(),
    )
    .unwrap();

    let lib_file = "src/lib.rs";
    let append_content = format!("mod {}; \n", title);
    let mut content = fs::read(lib_file).unwrap();
    content.append(&mut append_content.into_bytes());
    fs::write(lib_file, content).unwrap();
}
