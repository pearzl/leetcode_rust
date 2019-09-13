// q0208_implement_trie_prefix_tree

use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

/* following define works, but MaybeUninit still unstable on leetcode's complier */
// t: [Option<(Rc<RefCell<Box<Trie>>>, bool)>;26]
#[derive(Clone, Debug)]
struct Trie {
    t: Vec<Option<(Rc<RefCell<Box<Trie>>>, bool)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        /* following code works, but MaybeUninit still unstable on leetcode's complier */

        // use std::mem::MaybeUninit;

        // let t = unsafe {
        //     let mut tmp: [MaybeUninit<Option<(Rc<RefCell<Box<Trie>>>, bool)>>; 26] = MaybeUninit::uninit().assume_init();
        //     for m in tmp.iter_mut() {
        //         std::ptr::write(m.as_mut_ptr(), None);
        //     }
        //     std::mem::transmute(tmp)
        // };

        let mut t = Vec::with_capacity(26);
        for _ in 0..26 {
            t.push(None);
        }
        Trie { t }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let b = unsafe { Box::from_raw(self as *mut Trie) };
        let mut tree = Rc::new(RefCell::new(b));
        let root_tree = Rc::clone(&tree);
        let last_i = word.len() - 1;
        for (l, i) in word.chars().map(|c| (c as u8 - b'a') as usize).enumerate() {
            let tmp_tree = Rc::clone(&tree);
            {
                let trie_i = &mut tmp_tree.borrow_mut().t[i];
                if let Some(rrct_pair) = trie_i {
                    if last_i == l {
                        rrct_pair.1 = true;
                    }
                    tree = Rc::clone(&rrct_pair.0);
                } else {
                    let t = Rc::new(RefCell::new(Box::new(Trie::new())));
                    if last_i == l {
                        *trie_i = Some((Rc::clone(&t), true));
                    } else {
                        *trie_i = Some((Rc::clone(&t), false));
                    }
                    tree = t;
                }
            }
        }
        let rt = Rc::try_unwrap(root_tree).unwrap().into_inner();
        unsafe { std::ptr::swap(self as *mut Trie, Box::into_raw(rt)) }
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let last_i = word.len() - 1;
        let mut w = word.chars().map(|c| (c as u8 - b'a') as usize);
        let first_i = w.next().unwrap();
        match &self.t[first_i] {
            None => return false,
            Some((rrct, end)) => {
                let mut rrct_tree = Rc::clone(rrct);
                for (l, i) in w.enumerate() {
                    let tmp_tree = Rc::clone(&rrct_tree);
                    let tree = tmp_tree.borrow();
                    if let Some(ref t) = tree.t[i] {
                        if l == last_i - 1 {
                            return t.1;
                        } else {
                            rrct_tree = Rc::clone(&t.0);
                        }
                    } else {
                        return false;
                    }
                }
                // panic!("should return in for loop");
                // if following code run, then word has only one character.
                return *end;
            }
        }
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut w = prefix.chars().map(|c| (c as u8 - b'a') as usize);
        let first_i = w.next().unwrap();
        match &self.t[first_i] {
            None => return false,
            Some((rrct, _)) => {
                let mut rrct_tree = Rc::clone(rrct);
                while let Some(i) = w.next() {
                    let tmp_tree = Rc::clone(&rrct_tree);
                    let tree = tmp_tree.borrow();
                    if let Some(ref t) = tree.t[i] {
                        rrct_tree = Rc::clone(&t.0);
                    } else {
                        return false;
                    }
                }
                return true;
            }
        }
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn it_works() {
        let mut t = Trie::new();
        t.insert(String::from("app"));
        assert_eq!(true, t.search(String::from("app")));
        assert_eq!(false, t.search(String::from("apple")));
        assert_eq!(false, t.search(String::from("ap")));
        assert_eq!(true, t.starts_with(String::from("ap")));
        assert_eq!(true, t.starts_with(String::from("app")));
        assert_eq!(false, t.starts_with(String::from("apple")));

        let mut t = Trie::new();
        t.insert(String::from("apple"));
        t.insert(String::from("app"));
        assert_eq!(true, t.search(String::from("app")));
        assert_eq!(true, t.search(String::from("apple")));
    }
}
