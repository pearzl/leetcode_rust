// generate init TEMPLATE source;
// modify lib file.
// usage: cargo run <title> <num>

const TEMPLATE: &'static str = r###"

struct Solution;





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
use std::collections::HashSet;

fn main() {
    let mut args = env::args();
    args.next();
    let title = args.next().unwrap().replace("-", "_");
    let num: usize = args.next().unwrap().parse().unwrap();
    // let title = format!("q{:04}_{} ", num, title.trim_end_matches(|c: char| !(c.is_ascii_alphanumeric() || c == '_')));
    let title = format!("q{:04}_{}", num, title.trim());

    // fs::write(
    //     format!("src/answer/{}.rs", title),
    //     format!("// {} \n{}", title, TEMPLATE).as_bytes(),
    // )
    // .unwrap();

    let lib_file = "src/lib.rs";
    let append_content = format!("    mod {}; \n", title);
    let content = fs::read_to_string(lib_file).unwrap();
    let t: Vec<&str> = content.split(|c| c == '{' || c == '}').collect();
    let mut new_content = String::from(t[0]);
    new_content.push_str(" { \n");
    let mut mods: Vec<&str> = t[1].lines().filter(|s| s.trim() != "").collect();
    let index_check: HashSet<usize> = mods.iter().map(|s| s[9..13].parse::<usize>().unwrap()).collect();

    if index_check.contains(&num) {
        eprintln!("already exist!");
        return;
    }
    mods.push(&append_content);
    mods.sort();
    for m in mods.iter() {
        new_content.push_str(m);
        new_content.push('\n');
    }
    new_content.push_str("}\n");

    fs::write(
        format!("src/answer/{}.rs", title),
        format!("// {} \n{}", title, TEMPLATE).as_bytes(),
    )
    .unwrap();

    fs::write(lib_file, new_content).unwrap();
}
