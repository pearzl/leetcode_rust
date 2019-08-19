// q0146_lru_cache

use std::collections::HashMap;
use std::collections::VecDeque;
#[derive(Debug)]
struct LRUCache {
    cache: HashMap<i32, (usize, i32)>,
    queue: VecDeque<i32>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cache: HashMap::with_capacity(capacity as usize),
            queue: VecDeque::with_capacity(capacity as usize),
            capacity: capacity as usize,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(v) = self.cache.get_mut(&key) {
            self.queue.push_back(key);
            v.0 += 1;
            return v.1;
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(v) = self.cache.get_mut(&key) {
            v.0 += 1;
            v.1 = value;
            self.queue.push_back(key);
        } else {
            if self.cache.len() >= self.capacity {
                while let Some(k) = self.queue.pop_front() {
                    if let Some(v) = self.cache.get_mut(&k) {
                        if v.0 == 1 {
                            self.cache.remove(&k);
                            break;
                        } else {
                            v.0 -= 1;
                        }
                    }
                }
            }
            self.queue.push_back(key);
            if let Some(v) = self.cache.get_mut(&key) {
                v.0 += 1;
                v.1 = value;
            } else {
                self.cache.insert(key, (1, value));
            }
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    // use super::Solution;

    use super::LRUCache;

    fn run(actions: Vec<&str>, values: Vec<Vec<i32>>) -> String {
        let mut lru = LRUCache::new(1);
        let mut ret = vec![];
        for i in 0..actions.len() {
            if actions[i] == "LRUCache" {
                lru = LRUCache::new(values[i][0]);
                ret.push(String::from("null"));
            } else if actions[i] == "get" {
                print!("get({}): ", values[i][0]);
                let r = lru.get(values[i][0]);
                ret.push(format!("{}", r));
                println!("{:?}", lru);
            } else if actions[i] == "put" {
                print!("put({},{}): ", values[i][0], values[i][1]);
                lru.put(values[i][0], values[i][1]);
                ret.push(String::from("null"));
                println!("{:?}", lru);
            }
        }
        let mut s = String::from("[");
        for ss in ret.into_iter() {
            s.push_str(ss.as_str());
            s.push(',');
        }
        s.pop();
        s.push(']');
        s
    }

    #[test]
    fn it_works() {
        assert_eq!(
            "[null,null,null,2,null,null,-1]",
            run(
                vec!["LRUCache", "put", "put", "get", "put", "put", "get"],
                vec![
                    vec![2],
                    vec![2, 1],
                    vec![2, 2],
                    vec![2],
                    vec![1, 1],
                    vec![4, 1],
                    vec![2]
                ]
            )
        );

        // assert_eq!(
        //     "[null,null,null,null,null,-1,3]",
        //     run(
        //         vec!["LRUCache", "put", "put", "put", "put", "get", "get"],
        //         vec![
        //             vec![2],
        //             vec![2, 1],
        //             vec![1, 1],
        //             vec![2, 3],
        //             vec![4, 1],
        //             vec![1],
        //             vec![2]
        //         ]
        //     )
        // );

        // let s = run(
        //     vec!["LRUCache", "get", "put", "get", "put", "put", "get", "get"],
        //     vec![
        //         vec![2],
        //         vec![2],
        //         vec![2, 6],
        //         vec![1],
        //         vec![1, 5],
        //         vec![1, 2],
        //         vec![1],
        //         vec![2],
        //     ],
        // );
        // assert_eq!(s, "[null,-1,null,-1,null,null,2,6]");

        // run(
        //     vec!["LRUCache", "put", "put", "get", "put", "put", "get"],
        //     vec![
        //         vec![2],
        //         vec![2, 1],
        //         vec![2, 2],
        //         vec![2],
        //         vec![1, 1],
        //         vec![4, 1],
        //         vec![2],
        //     ],
        // );

        // run(
        //     vec![
        //         "LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get",
        //     ],
        //     vec![
        //         vec![2],
        //         vec![1, 1],
        //         vec![2, 2],
        //         vec![1],
        //         vec![3, 3],
        //         vec![2],
        //         vec![4, 4],
        //         vec![1],
        //         vec![3],
        //         vec![4],
        //     ],
        // );

        // run(
        //     vec![
        //         "LRUCache", "put", "put", "put", "put", "put", "get", "put", "get", "get", "put",
        //         "get", "put", "put", "put", "get", "put", "get", "get", "get", "get", "put", "put",
        //         "get", "get", "get", "put", "put", "get", "put", "get", "put", "get", "get", "get",
        //         "put", "put", "put", "get", "put", "get", "get", "put", "put", "get", "put", "put",
        //         "put", "put", "get", "put", "put", "get", "put", "put", "get", "put", "put", "put",
        //         "put", "put", "get", "put", "put", "get", "put", "get", "get", "get", "put", "get",
        //         "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "get", "get",
        //         "get", "put", "put", "put", "get", "put", "put", "put", "get", "put", "put", "put",
        //         "get", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put",
        //         "put", "put", "put",
        //     ],
        //     vec![
        //         vec![10],
        //         vec![10, 13],
        //         vec![3, 17],
        //         vec![6, 11],
        //         vec![10, 5],
        //         vec![9, 10],
        //         vec![13],
        //         vec![2, 19],
        //         vec![2],
        //         vec![3],
        //         vec![5, 25],
        //         vec![8],
        //         vec![9, 22],
        //         vec![5, 5],
        //         vec![1, 30],
        //         vec![11],
        //         vec![9, 12],
        //         vec![7],
        //         vec![5],
        //         vec![8],
        //         vec![9],
        //         vec![4, 30],
        //         vec![9, 3],
        //         vec![9],
        //         vec![10],
        //         vec![10],
        //         vec![6, 14],
        //         vec![3, 1],
        //         vec![3],
        //         vec![10, 11],
        //         vec![8],
        //         vec![2, 14],
        //         vec![1],
        //         vec![5],
        //         vec![4],
        //         vec![11, 4],
        //         vec![12, 24],
        //         vec![5, 18],
        //         vec![13],
        //         vec![7, 23],
        //         vec![8],
        //         vec![12],
        //         vec![3, 27],
        //         vec![2, 12],
        //         vec![5],
        //         vec![2, 9],
        //         vec![13, 4],
        //         vec![8, 18],
        //         vec![1, 7],
        //         vec![6],
        //         vec![9, 29],
        //         vec![8, 21],
        //         vec![5],
        //         vec![6, 30],
        //         vec![1, 12],
        //         vec![10],
        //         vec![4, 15],
        //         vec![7, 22],
        //         vec![11, 26],
        //         vec![8, 17],
        //         vec![9, 29],
        //         vec![5],
        //         vec![3, 4],
        //         vec![11, 30],
        //         vec![12],
        //         vec![4, 29],
        //         vec![3],
        //         vec![9],
        //         vec![6],
        //         vec![3, 4],
        //         vec![1],
        //         vec![10],
        //         vec![3, 29],
        //         vec![10, 28],
        //         vec![1, 20],
        //         vec![11, 13],
        //         vec![3],
        //         vec![3, 12],
        //         vec![3, 8],
        //         vec![10, 9],
        //         vec![3, 26],
        //         vec![8],
        //         vec![7],
        //         vec![5],
        //         vec![13, 17],
        //         vec![2, 27],
        //         vec![11, 15],
        //         vec![12],
        //         vec![9, 19],
        //         vec![2, 15],
        //         vec![3, 16],
        //         vec![1],
        //         vec![12, 17],
        //         vec![9, 1],
        //         vec![6, 19],
        //         vec![4],
        //         vec![5],
        //         vec![5],
        //         vec![8, 1],
        //         vec![11, 7],
        //         vec![5, 2],
        //         vec![9, 28],
        //         vec![1],
        //         vec![2, 2],
        //         vec![7, 4],
        //         vec![4, 22],
        //         vec![7, 24],
        //         vec![9, 26],
        //         vec![13, 28],
        //         vec![11, 26],
        //     ],
        // );
    }
}
