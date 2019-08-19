// q0149_max_points_on_a_line

struct Solution;

// use std::time::Instant;
// use std::collections::HashMap;
// impl Solution {
//     pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
//         // let begin = Instant::now();
//         let points_num = points.len();
//         if points_num <= 1 {
//             return points_num as i32;
//         }
//         let mut map = HashMap::new();
//         // println!("begin: {}", begin.elapsed().as_nanos());
//         for i in 0..points_num-1 {
//             for j in 1..points_num {
//                 let p1 = (points[i][0], points[i][1]);
//                 let p2 = (points[j][0], points[j][1]);
//                 let line = Line::new(p1, p2);
//                 map.insert(line, 0);
//             }
//         }
//         // println!("build line: {}", begin.elapsed().as_nanos());
//         for p in points.iter() {
//             let p = (p[0], p[1]);
//             for (l, c) in map.iter_mut() {
//                 if l.contains(p) {
//                     *c += 1;
//                 }
//             }
//         }
//         // println!("loop point: {}", begin.elapsed().as_nanos());
//         let mut max_count = 0;
//         for (_, c) in map.iter() {
//             max_count = max_count.max(*c);
//         }
//         // println!("end: {}", begin.elapsed().as_nanos());
//         max_count as i32
//     }
// }

// #[derive(Debug, Hash, Eq, PartialEq)]
// struct Line {
//     rate: Option<(i32, i32)>,
//     point: (i32, i32)
// }

// impl Line {
//     pub fn new(p1: (i32, i32), p2: (i32, i32)) -> Self {
//         Line{
//             rate: Line::rate(p1, p2),
//             point: p1
//         }
//     }

//     pub fn contains(&self, p: (i32, i32)) -> bool {
//         if self.point == p {
//             return true
//         }
//         self.rate == Line::rate(self.point, p)
//     }

//     fn rate(mut p1: (i32, i32), mut p2: (i32, i32)) -> Option<(i32, i32)> {
//         if p1 == p2 {
//             None
//         }else if p1.0 == p2.0 {
//             Some((p1.0, 0))
//         }else if p1.1 == p2.1 {
//             Some((0, p1.1))
//         }else {
//             let begin = Instant::now();
//             println!("{:?}-{:?}", p1, p2);
//             if p1.0 > p2.0 {
//                 std::mem::swap(&mut p1, &mut p2);
//             }
//             let mut roc = (p2.0-p1.0, p2.1-p1.1);
//             let mut common_divisor = roc.0.min(i32::abs(roc.1));
//             println!("begin: {}", begin.elapsed().as_nanos());
//             while common_divisor != 1 {
//                 // println!("{:?}--{}", roc, common_divisor);
//                 if roc.0 % common_divisor == 0 && roc.1 % common_divisor == 0 {
//                     roc = (roc.0 / common_divisor, roc.1 / common_divisor);
//                     // common_divisor = i32::abs(roc.0).min(i32::abs(roc.1));
//                     break;
//                 }else {
//                     common_divisor -= 1;
//                 }
//             }
//             println!("done: {}", begin.elapsed().as_nanos());
//             // let mut common_divisor = 2;
//             // while common_divisor != roc.0.min(i32::abs(roc.1)) + 1 {
//             //     if roc.0 % common_divisor == 0 && roc.1 % common_divisor == 0 {
//             //         roc = (roc.0 / common_divisor, roc.1 / common_divisor);
//             //         common_divisor = 2;
//             //     }else {
//             //         common_divisor += 1;
//             //     }
//             // }
//             Some(roc)
//         }
//     }
// }

// use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        // let begin = Instant::now();
        let points_num = points.len();
        if points_num <= 1 {
            return points_num as i32;
        }
        let mut map: HashMap<Line, HashSet<usize>> = HashMap::new();
        let mut rate_cal = RateCalculator::new();
        for i in 0..points_num - 1 {
            for j in i + 1..points_num {
                let p1 = Point {
                    x: points[i][0],
                    y: points[i][1],
                };
                let p2 = Point {
                    x: points[j][0],
                    y: points[j][1],
                };
                let line = Line {
                    rate: rate_cal.calculate(p1, p2),
                    point: p1,
                };
                let mut new_line = true;
                for (l, set) in map.iter_mut() {
                    if l.rate != line.rate {
                        continue;
                    }
                    if l.point == line.point || rate_cal.calculate(l.point, line.point) == l.rate {
                        set.insert(i);
                        set.insert(j);
                        new_line = false;
                    }
                }
                if new_line {
                    let mut set = HashSet::with_capacity(2);
                    set.insert(i);
                    set.insert(j);
                    map.insert(line, set);
                }
                // println!("{:?}-{:?}->{:?}", p1, p2, map);
            }
        }
        let mut max_count = 0;
        for (_, v) in map.iter() {
            max_count = max_count.max(v.len() as i32);
        }
        max_count
    }
}

#[derive(Debug)]
struct RateCalculator {
    buf: HashMap<(Point, Point), Option<Rate>>,
}

impl RateCalculator {
    fn new() -> Self {
        RateCalculator {
            buf: HashMap::new(),
        }
    }

    fn calculate(&mut self, pp1: Point, pp2: Point) -> Option<Rate> {
        let p1 = pp1.min(pp2);
        let p2 = pp1.max(pp2);
        if let Some(ret) = self.buf.get(&(p1, p2)) {
            return *ret;
        } else {
            let ret;
            if p1 == p2 {
                ret = None
            } else if p1.x == p2.x {
                ret = Some(Rate { x: p1.x, y: 0 })
            } else if p1.y == p2.y {
                ret = Some(Rate { x: 0, y: p1.y })
            } else {
                let mut roc = Rate {
                    x: p2.x - p1.x,
                    y: p2.y - p1.y,
                };
                let mut common_divisor = roc.x.min(i32::abs(roc.y));
                while common_divisor != 1 {
                    if roc.x % common_divisor == 0 && roc.y % common_divisor == 0 {
                        roc.x /= common_divisor;
                        roc.y /= common_divisor;
                        break;
                    } else {
                        common_divisor -= 1;
                    }
                }
                ret = Some(roc)
            }
            self.buf.insert((p1, p2), ret);
            return ret;
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Line {
    rate: Option<Rate>,
    point: Point,
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Rate {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!( 3, Solution::max_points(vec![vec![1,1],vec![2,2],vec![3,3]]));
        // assert_eq!( 4, Solution::max_points(vec![vec![1,1],vec![3,2],vec![5,3],vec![4,1],vec![2,3],vec![1,4]]));
        // assert_eq!( 0, Solution::max_points(vec![]));
        // assert_eq!( 3, Solution::max_points(vec![vec![0,0],vec![1,1],vec![0,0]]));
        // assert_eq!( 2, Solution::max_points(vec![vec![0,0],vec![0,0]]));
        // assert_eq!( 3, Solution::max_points(vec![vec![1,1],vec![1,1],vec![2,3]]));
        assert_eq!(
            24,
            Solution::max_points(vec![
                vec![-240, -657],
                vec![-27, -188],
                vec![-616, -247],
                vec![-264, -311],
                vec![-352, -393],
                vec![-270, -748],
                vec![3, 4],
                vec![-308, -87],
                vec![150, 526],
                vec![0, -13],
                vec![-7, -40],
                vec![-3, -10],
                vec![-531, -892],
                vec![-88, -147],
                vec![4, -3],
                vec![-873, -555],
                vec![-582, -360],
                vec![-539, -207],
                vec![-118, -206],
                vec![970, 680],
                vec![-231, -47],
                vec![352, 263],
                vec![510, 143],
                vec![295, 480],
                vec![-590, -990],
                vec![-236, -402],
                vec![308, 233],
                vec![-60, -111],
                vec![462, 313],
                vec![-270, -748],
                vec![-352, -393],
                vec![-35, -148],
                vec![-7, -40],
                vec![440, 345],
                vec![388, 290],
                vec![270, 890],
                vec![10, -7],
                vec![60, 253],
                vec![-531, -892],
                vec![388, 290],
                vec![-388, -230],
                vec![340, 85],
                vec![0, -13],
                vec![770, 473],
                vec![0, 73],
                vec![873, 615],
                vec![-42, -175],
                vec![-6, -8],
                vec![49, 176],
                vec![308, 222],
                vec![170, 27],
                vec![-485, -295],
                vec![170, 27],
                vec![510, 143],
                vec![-18, -156],
                vec![-63, -316],
                vec![-28, -121],
                vec![396, 304],
                vec![472, 774],
                vec![-14, -67],
                vec![-5, 7],
                vec![-485, -295],
                vec![118, 186],
                vec![-154, -7],
                vec![-7, -40],
                vec![-97, -35],
                vec![4, -9],
                vec![-18, -156],
                vec![0, -31],
                vec![-9, -124],
                vec![-300, -839],
                vec![-308, -352],
                vec![-425, -176],
                vec![-194, -100],
                vec![873, 615],
                vec![413, 676],
                vec![-90, -202],
                vec![220, 140],
                vec![77, 113],
                vec![-236, -402],
                vec![-9, -124],
                vec![63, 230],
                vec![-255, -118],
                vec![472, 774],
                vec![-56, -229],
                vec![90, 228],
                vec![3, -8],
                vec![81, 196],
                vec![970, 680],
                vec![485, 355],
                vec![-354, -598],
                vec![-385, -127],
                vec![-2, 7],
                vec![531, 872],
                vec![-680, -263],
                vec![-21, -94],
                vec![-118, -206],
                vec![616, 393],
                vec![291, 225],
                vec![-240, -657],
                vec![-5, -4],
                vec![1, -2],
                vec![485, 355],
                vec![231, 193],
                vec![-88, -147],
                vec![-291, -165],
                vec![-176, -229],
                vec![154, 153],
                vec![-970, -620],
                vec![-77, 33],
                vec![-60, -111],
                vec![30, 162],
                vec![-18, -156],
                vec![425, 114],
                vec![-177, -304],
                vec![-21, -94],
                vec![-10, 9],
                vec![-352, -393],
                vec![154, 153],
                vec![-220, -270],
                vec![44, -24],
                vec![-291, -165],
                vec![0, -31],
                vec![240, 799],
                vec![-5, -9],
                vec![-70, -283],
                vec![-176, -229],
                vec![3, 8],
                vec![-679, -425],
                vec![-385, -127],
                vec![396, 304],
                vec![-308, -352],
                vec![-595, -234],
                vec![42, 149],
                vec![-220, -270],
                vec![385, 273],
                vec![-308, -87],
                vec![-54, -284],
                vec![680, 201],
                vec![-154, -7],
                vec![-440, -475],
                vec![-531, -892],
                vec![-42, -175],
                vec![770, 473],
                vec![118, 186],
                vec![-385, -127],
                vec![154, 153],
                vec![56, 203],
                vec![-616, -247]
            ])
        );
    }

    // #[test]
    // fn rate_of_change() {
    //     assert_eq!((2,1), Solution::rate_of_change((3,2), (1,1)));
    // }
}
