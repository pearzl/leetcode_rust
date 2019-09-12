// q0208_implement_trie_prefix_tree 

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Trie {
    t: [Option<(Rc<RefCell<Trie>>, bool)>;26]
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        use std::mem::MaybeUninit;

        let t = unsafe {
            let mut tmp: [MaybeUninit<Option<(Rc<RefCell<Trie>>, bool)>>; 26] = MaybeUninit::uninit().assume_init();
            for m in tmp.iter_mut() {
                std::ptr::write(m.as_mut_ptr(), None);
            }
            std::mem::transmute(tmp)
        };
        Trie{t}
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        use std::ops::DerefMut;
        let mut tree = Rc::new(RefCell::new(self.clone()));
        let root_tree = Rc::clone(&tree);
        let last_i = word.len() - 1;
        for (l, i)in word.chars().map(|c| (c as u8 - b'a') as usize).enumerate() {
            let tmp_tree = Rc::clone(&tree);
            {let trie_i = &mut tmp_tree.borrow_mut().t[i];
            if let Some(rrct_pair) = trie_i {
                tree = Rc::clone(&rrct_pair.0);
            }else {
                let t = Rc::new(RefCell::new(Trie::new()));
                if last_i == l {
                    *trie_i = Some((Rc::clone(&t), true));
                }else{
                    *trie_i = Some((Rc::clone(&t), false));
                }
                tree = t;
            }}
        }
        let mut tmp = root_tree.borrow_mut();
        std::mem::swap(self, tmp.deref_mut())
    }
    
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        true
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        true
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
        println!("{:?}", t);
        t.insert(String::from("app"));
        println!("{:?}", t);
    }
}
