fn find_median(nums: &[i32]) -> f64 {
    let len: usize = nums.len();
    let mid: usize = len / 2;
    print!("mid = {} ", mid);
    print!("mid - 1 = {} ", mid-1);
    if len % 2 == 0 {
        // If the array length is even
        (nums[mid - 1] as f64 + nums[mid] as f64) / 2.0
    } else {
        // If the array length is odd
        nums[mid] as f64
    }
}

fn main() {
    let nums: [i32; 6] = [1, 2, 3, 4, 5, 6];
    print!("The median is {}", find_median(&nums));
}
