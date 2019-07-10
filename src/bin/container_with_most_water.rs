//11

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;
    for i in 0..height.len() {
        let mut j = i + 1;
        while j < height.len() {
            let area = (j - i) as i32 * height[i].min(height[j]);
            max_area = max_area.max(area);
            j += 1;
        }
    }
    max_area
}
