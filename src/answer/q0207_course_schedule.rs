// q0206_course_schedule

struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut precondition: Vec<HashSet<i32>> = Vec::with_capacity(num_courses);
        for _ in 0..num_courses {
            precondition.push(HashSet::new());
        }
        for p in prerequisites.into_iter() {
            let course = p[0] as usize;
            let precourse = p[1];
            precondition[course].insert(precourse);
        }
        let mut finished_courses: HashSet<i32> = HashSet::with_capacity(num_courses);
        let mut unfinished_courses: HashMap<i32, HashSet<i32>> =
            HashMap::with_capacity(num_courses);
        for (i, c) in precondition.into_iter().enumerate() {
            if c.len() == 0 {
                finished_courses.insert(i as i32);
            } else {
                unfinished_courses.insert(i as i32, c);
            }
        }
        let mut is_changed = true;
        while is_changed {
            is_changed = false;
            let mut need_remove_courses: Vec<i32> = vec![];
            for (k, ufcp_cond) in unfinished_courses.iter_mut() {
                let mut need_remove_pre_courses: Vec<i32> = vec![];
                for pc in ufcp_cond.iter().cloned() {
                    if finished_courses.contains(&pc) {
                        is_changed = true;
                        need_remove_pre_courses.push(pc);
                    }
                }
                for i in need_remove_pre_courses.iter().cloned() {
                    ufcp_cond.remove(&i);
                }
                if ufcp_cond.is_empty() {
                    is_changed = true;
                    need_remove_courses.push(*k);
                }
            }
            for i in need_remove_courses.iter() {
                unfinished_courses.remove(i);
                finished_courses.insert(*i);
            }
        }
        unfinished_courses.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!( true, Solution::can_finish(2, vec![vec![1,0]]));
        // assert_eq!( false, Solution::can_finish(2, vec![vec![1,0], vec![0,1]]));
        // assert_eq!( false, Solution::can_finish(3, vec![vec![1,0], vec![0,1]]));
        // assert_eq!( false, Solution::can_finish(3, vec![vec![1,0], vec![0,1]]));
        // assert_eq!( false, Solution::can_finish(4, vec![vec![1,0], vec![0,1], vec![3, 0]]));
        // assert_eq!( false, Solution::can_finish(3, vec![vec![1,0], vec![0,1], vec![2, 0]]));
        assert_eq!(
            true,
            Solution::can_finish(4, vec![vec![1, 0], vec![2, 1], vec![3, 2]])
        );
    }
}
