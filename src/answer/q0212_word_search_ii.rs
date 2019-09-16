// q0212_word_search_ii

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        for s in words.iter() {
            trie.insert(s.as_str());
        }
        // println!("{:?}", trie);
        let mut ret = HashSet::new();
        for (init_pos_line, r) in board.iter().enumerate() {
            for (init_pos_row, _) in r.iter().enumerate() {
                let mut paths = vec![Path::start_from(&board, (init_pos_line, init_pos_row))];
                while !paths.is_empty() {
                    let mut tmp = vec![];
                    for p in paths.iter() {
                        let s = p.get_value();
                        match trie.start_with(s.as_str()) {
                            SearchResult::FOUND => {
                                ret.insert(s);
                                tmp.append(&mut p.forward());
                            }
                            SearchResult::MISS => {
                                continue;
                            }
                            SearchResult::PREFIX => {
                                tmp.append(&mut p.forward());
                            }
                        }
                    }
                    std::mem::replace(&mut paths, tmp);
                }
            }
        }
        ret.into_iter().collect()
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Path<'a> {
    board: &'a Vec<Vec<char>>,
    boundary: (usize, usize),
    paths: Vec<(usize, usize)>,
}

impl<'a> Path<'a> {
    pub fn start_from(board: &'a Vec<Vec<char>>, pos: (usize, usize)) -> Self {
        let line = board.len();
        let row = board[0].len();
        let boundary = (line - 1, row - 1);
        assert!(pos.0 < line && pos.1 < row);
        let paths = vec![pos];
        Path {
            board,
            boundary,
            paths,
        }
    }

    pub fn get_value(&self) -> String {
        let mut s = Vec::with_capacity(self.paths.len());
        for (l, r) in self.paths.iter() {
            let t = self.board[*l][*r] as u8;
            s.push(t);
        }
        String::from_utf8(s).unwrap()
    }

    pub fn forward(&self) -> Vec<Self> {
        let stand_pos = self.paths.last().unwrap();
        let mut ret = Vec::with_capacity(4);
        macro_rules! next_pos {
            ($condition: expr, $value: expr) => {
                if $condition {
                    let b = $value;
                    if !self.paths.contains(&b) {
                        let mut t = self.paths.clone();
                        t.push(b);
                        ret.push(Path { paths: t, ..*self });
                    }
                }
            };
        };
        next_pos!(stand_pos.0 != 0, (stand_pos.0 - 1, stand_pos.1));
        next_pos!(
            stand_pos.0 != self.boundary.0,
            (stand_pos.0 + 1, stand_pos.1)
        );
        next_pos!(stand_pos.1 != 0, (stand_pos.0, stand_pos.1 - 1));
        next_pos!(
            stand_pos.1 != self.boundary.1,
            (stand_pos.0, stand_pos.1 + 1)
        );
        ret
    }
}

#[derive(Debug)]
struct Trie {
    entry: HashMap<char, (bool, Rc<RefCell<Trie>>)>,
}

#[derive(PartialEq, Eq, Debug)]
enum SearchResult {
    PREFIX,
    FOUND,
    MISS,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            entry: HashMap::with_capacity(26),
        }
    }

    pub fn insert(&mut self, s: &str) {
        let ss = s.as_bytes();
        let node = self
            .entry
            .entry(ss[0] as char)
            .or_insert_with(|| (false, Rc::new(RefCell::new(Trie::new()))));
        let len = ss.len();
        if len == 1 {
            node.0 = true;
            return;
        }
        let mut rrct = Rc::clone(&node.1);
        for i in 1..len {
            let tmp = Rc::clone(&rrct);
            let mut t = tmp.borrow_mut();
            let tmp_node = t
                .entry
                .entry(*(&ss[i]) as char)
                .or_insert_with(|| (false, Rc::new(RefCell::new(Trie::new()))));
            if i == len - 1 {
                tmp_node.0 = true;
            } else {
                rrct = Rc::clone(&tmp_node.1);
            }
        }
    }

    pub fn start_with(&self, s: &str) -> SearchResult {
        let ss = s.as_bytes();
        if let Some(node) = self.entry.get(&(ss[0] as char)) {
            let len = ss.len();
            if len == 1 {
                if node.0 {
                    return SearchResult::FOUND;
                } else {
                    return SearchResult::PREFIX;
                }
            }
            let mut rrct = Rc::clone(&node.1);
            for i in 1..len {
                let tmp = Rc::clone(&rrct);
                let t = tmp.borrow();
                if let Some(n) = t.entry.get(&(ss[i] as char)) {
                    if i == len - 1 {
                        if n.0 {
                            return SearchResult::FOUND;
                        } else {
                            return SearchResult::PREFIX;
                        }
                    } else {
                        rrct = Rc::clone(&n.1);
                    }
                } else {
                    return SearchResult::MISS;
                }
            }
            panic!("should return in for loop");
        } else {
            return SearchResult::MISS;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::{Path, SearchResult, Trie};
    use crate::util;

    #[test]
    fn it_works() {
        assert_eq!(
            util::vec_2_set(vec![String::from("eat"), String::from("oath")]),
            util::vec_2_set(Solution::find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v']
                ],
                vec!["oath", "pea", "eat", "rain"]
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            )),
        );
    }

    #[test]
    fn test_trie() {
        let mut t = Trie::new();
        t.insert("app");
        assert_eq!(SearchResult::PREFIX, t.start_with("ap"));
        assert_eq!(SearchResult::FOUND, t.start_with("app"));
        assert_eq!(SearchResult::MISS, t.start_with("apple"));

        let mut t = Trie::new();
        t.insert("aba");
        t.insert("baa");
        t.insert("bab");
        t.insert("aaab");
        t.insert("aaa");
        t.insert("aaaa");
        t.insert("aaba");
        assert_eq!(SearchResult::PREFIX, t.start_with("a"));
        assert_eq!(SearchResult::PREFIX, t.start_with("ab"));
        assert_eq!(SearchResult::PREFIX, t.start_with("aa"));
        assert_eq!(SearchResult::FOUND, t.start_with("aba"));
        assert_eq!(SearchResult::FOUND, t.start_with("aaa"));
        assert_eq!(SearchResult::MISS, t.start_with("abaa"));
        assert_eq!(SearchResult::FOUND, t.start_with("aaab"));
    }

    #[test]
    fn test_path() {
        fn step(cur: Vec<Path>) -> Vec<Path> {
            let mut r = vec![];
            for p in cur {
                r.append(&mut p.forward());
            }
            r
        }
        fn check(cur: &Vec<Path>, ret: Vec<&str>) {
            for (p, r) in cur.into_iter().zip(ret) {
                assert_eq!(p.get_value(), r)
            }
        }

        {
            let board = vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v'],
            ];

            let init_path = Path::start_from(&board, (0, 0));
            assert_eq!("o", init_path.get_value());

            let step1 = init_path.forward();
            assert_eq!(2, step1.len());
            check(&step1, vec!["oe", "oa"]);

            let step2 = step(step1);
            assert_eq!(4, step2.len());
            check(&step2, vec!["oei", "oet", "oat", "oaa"]);

            let step3 = step(step2);
            assert_eq!(10, step3.len());
            check(
                &step3,
                vec![
                    "oeii", "oeih", "oeta", "oeth", "oeta", "oath", "oate", "oata", "oaaa", "oaan",
                ],
            );
        }

        {
            let board = vec![vec!['a', 'b'], vec!['a', 'a']];

            let init_path = Path::start_from(&board, (1, 1));
            assert_eq!("a", init_path.get_value());

            let step1 = init_path.forward();
            assert_eq!(2, step1.len());
            check(&step1, vec!["ab", "aa"]);

            let step2 = step(step1);
            assert_eq!(2, step2.len());
            check(&step2, vec!["aba", "aaa"]);

            let step3 = step(step2);
            assert_eq!(2, step3.len());
            check(&step3, vec!["abaa", "aaab"]);
        }
    }
}
