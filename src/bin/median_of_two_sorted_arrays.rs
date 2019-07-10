// 4
fn main() {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let mut merge = Merge::new(nums1, nums2);
        if (m + n) % 2 == 1 {
            let index = (m + n - 1) / 2;
            let mut i = 0;
            let mut t = 0;
            while i <= index {
                t = merge.next();
                // println!("{},{},{}", t,i,index);
                i += 1;
            }
            return t as f64;
        } else {
            let index = (m + n - 2) / 2;
            let mut i = 0;
            let mut t = 0;
            while i <= index {
                t = merge.next();
                i += 1;
            }
            return (t + merge.next()) as f64 / 2.0;
        }
    }
}

struct Merge {
    nums1: Vec<i32>,
    i1: usize,
    nums2: Vec<i32>,
    i2: usize,
}

impl Merge {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        Merge {
            nums1: nums1,
            i1: 0,
            nums2: nums2,
            i2: 0,
        }
    }

    fn next(&mut self) -> i32 {
        if self.i1 == self.nums1.len() {
            self.i2 += 1;
            return self.nums2[self.i2 - 1];
        }
        if self.i2 == self.nums2.len() {
            self.i1 += 1;
            return self.nums1[self.i1 - 1];
        }
        if self.nums1[self.i1] < self.nums2[self.i2] {
            self.i1 += 1;
            return self.nums1[self.i1 - 1];
        } else {
            self.i2 += 1;
            return self.nums2[self.i2 - 1];
        }
    }
}
