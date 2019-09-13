// q0210_course_schedule_ii

struct Solution;

// use std::collections::{HashMap, HashSet};
// impl Solution {
//     pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
//         let mut pre_conditions = HashMap::with_capacity(num_courses as usize);
//         for prerequisity in prerequisites.into_iter() {
//             let set = pre_conditions
//                 .entry(prerequisity[0])
//                 .or_insert_with(|| HashSet::new());
//             set.insert(prerequisity[1]);
//         }
//         // println!("{:?}", pre_conditions);

//         let mut ret = Vec::with_capacity(num_courses as usize);
//         let mut begin_lesson = 0;
//         loop {
//             let mut learned_a_lesson = false;
//             for i in begin_lesson..num_courses {
//                 if ret.contains(&i) {
//                     // lesson i already learned.
//                     continue;
//                 }

//                 if let Some(set) = pre_conditions.get(&i) {
//                     let mut need_learn_other = false;
//                     for pre_lesson in set.iter() {
//                         if !ret.contains(pre_lesson) {
//                             need_learn_other = true;
//                             break;
//                         }
//                     }
//                     if need_learn_other {
//                         continue;
//                     }
//                 }

//                 ret.push(i);
//                 learned_a_lesson = true;
//                 break;
//             }
//             if learned_a_lesson {
//                 if ret.len() == num_courses as usize {
//                     // all lesson finished.
//                     break;
//                 } else {
//                     begin_lesson = 0; // choose a new lesson.
//                 }
//             } else {
//                 loop {
//                     if let Some(lesson) = ret.pop() {
//                         // wrong learn route map
//                         if lesson == num_courses - 1 {
//                             continue;
//                         } else {
//                             begin_lesson = lesson + 1;
//                             break;
//                         }
//                     } else {
//                         return vec![];
//                     }
//                 }
//             }
//         }
//         ret
//     }
// }

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut pre_conditions = HashMap::with_capacity(num_courses);
        for prerequisity in prerequisites.iter() {
            let set = pre_conditions
                .entry(prerequisity[0])
                .or_insert_with(|| HashSet::new());
            set.insert(prerequisity[1]);
        }
        // println!("{:?}", pre_conditions);

        let mut post_conditions = HashMap::with_capacity(num_courses);
        for postcondition in prerequisites.iter() {
            let set = post_conditions.entry(postcondition[1]).or_insert_with(||HashSet::new());
            set.insert(postcondition[0]);
        }

        let mut ret = vec![];
        let mut learnd_lessons = vec![false; num_courses];
        loop {
            let mut new_learn_lessons = vec![];
            for lesson_i in 0..num_courses {
                if learnd_lessons[lesson_i] {
                    continue;
                }

                if let Some(set) = pre_conditions.get(&(lesson_i as i32)) {
                    let mut need_learn_other = false;
                    for pre_lesson in set.iter() {
                        if !learnd_lessons[*pre_lesson as usize] {
                            need_learn_other = true;
                            break;
                        }
                    }
                    if need_learn_other {
                        continue;
                    }
                }

                new_learn_lessons.push(lesson_i as i32);
                learnd_lessons[lesson_i] = true;
            }
            if new_learn_lessons.is_empty() {
                break;
            }
            ret.append(&mut new_learn_lessons);
        }
        if ret.len() == num_courses {
            return ret;
        }else {
            return vec![];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(vec![1, 0], Solution::find_order(2, vec![vec![0, 1]]));
        assert_eq!(vec![0, 1], Solution::find_order(2, vec![vec![1, 0]]));
        assert!(vec![vec![0, 1], vec![1, 0]].contains(&Solution::find_order(2, vec![])));
        assert!(
            vec![vec![0, 1, 2, 3], vec![0, 2, 1, 3]].contains(&Solution::find_order(
                4,
                vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]
            ))
        );
        
        // assert!(vec![vec![0, 1], vec![1, 0]].contains(&Solution::find_order(
        //     100,
        //     vec![
        //         vec![6, 27],
        //         vec![83, 9],
        //         vec![10, 95],
        //         vec![48, 67],
        //         vec![5, 71],
        //         vec![18, 72],
        //         vec![7, 10],
        //         vec![92, 4],
        //         vec![68, 84],
        //         vec![6, 41],
        //         vec![82, 41],
        //         vec![18, 54],
        //         vec![0, 2],
        //         vec![1, 2],
        //         vec![8, 65],
        //         vec![47, 85],
        //         vec![39, 51],
        //         vec![13, 78],
        //         vec![77, 50],
        //         vec![70, 56],
        //         vec![5, 61],
        //         vec![26, 56],
        //         vec![18, 19],
        //         vec![35, 49],
        //         vec![79, 53],
        //         vec![40, 22],
        //         vec![8, 19],
        //         vec![60, 56],
        //         vec![48, 50],
        //         vec![20, 70],
        //         vec![35, 12],
        //         vec![99, 85],
        //         vec![12, 75],
        //         vec![2, 36],
        //         vec![36, 22],
        //         vec![21, 15],
        //         vec![98, 1],
        //         vec![34, 94],
        //         vec![25, 41],
        //         vec![65, 17],
        //         vec![1, 56],
        //         vec![43, 96],
        //         vec![74, 57],
        //         vec![19, 62],
        //         vec![62, 78],
        //         vec![50, 86],
        //         vec![46, 22],
        //         vec![10, 13],
        //         vec![47, 18],
        //         vec![20, 66],
        //         vec![83, 66],
        //         vec![51, 47],
        //         vec![23, 66],
        //         vec![87, 42],
        //         vec![25, 81],
        //         vec![60, 81],
        //         vec![25, 93],
        //         vec![35, 89],
        //         vec![65, 92],
        //         vec![87, 39],
        //         vec![12, 43],
        //         vec![75, 73],
        //         vec![28, 96],
        //         vec![47, 55],
        //         vec![18, 11],
        //         vec![29, 58],
        //         vec![78, 61],
        //         vec![62, 75],
        //         vec![60, 77],
        //         vec![13, 46],
        //         vec![97, 92],
        //         vec![4, 64],
        //         vec![91, 47],
        //         vec![58, 66],
        //         vec![72, 74],
        //         vec![28, 17],
        //         vec![29, 98],
        //         vec![53, 66],
        //         vec![37, 5],
        //         vec![38, 12],
        //         vec![44, 98],
        //         vec![24, 31],
        //         vec![68, 23],
        //         vec![86, 52],
        //         vec![79, 49],
        //         vec![32, 25],
        //         vec![90, 18],
        //         vec![16, 57],
        //         vec![60, 74],
        //         vec![81, 73],
        //         vec![26, 10],
        //         vec![54, 26],
        //         vec![57, 58],
        //         vec![46, 47],
        //         vec![66, 54],
        //         vec![52, 25],
        //         vec![62, 91],
        //         vec![6, 72],
        //         vec![81, 72],
        //         vec![50, 35],
        //         vec![59, 87],
        //         vec![21, 3],
        //         vec![4, 92],
        //         vec![70, 12],
        //         vec![48, 4],
        //         vec![9, 23],
        //         vec![52, 55],
        //         vec![43, 59],
        //         vec![49, 26],
        //         vec![25, 90],
        //         vec![52, 0],
        //         vec![55, 8],
        //         vec![7, 23],
        //         vec![97, 41],
        //         vec![0, 40],
        //         vec![69, 47],
        //         vec![73, 68],
        //         vec![10, 6],
        //         vec![47, 9],
        //         vec![64, 24],
        //         vec![95, 93],
        //         vec![79, 66],
        //         vec![77, 21],
        //         vec![80, 69],
        //         vec![85, 5],
        //         vec![24, 48],
        //         vec![74, 31],
        //         vec![80, 76],
        //         vec![81, 27],
        //         vec![71, 94],
        //         vec![47, 82],
        //         vec![3, 24],
        //         vec![66, 61],
        //         vec![52, 13],
        //         vec![18, 38],
        //         vec![1, 35],
        //         vec![32, 78],
        //         vec![7, 58],
        //         vec![26, 58],
        //         vec![64, 47],
        //         vec![60, 6],
        //         vec![62, 5],
        //         vec![5, 22],
        //         vec![60, 54],
        //         vec![49, 40],
        //         vec![11, 56],
        //         vec![19, 85],
        //         vec![65, 58],
        //         vec![88, 44],
        //         vec![86, 58]
        //     ]
        // )));
    }
}
