fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = 0;
    let mut current_sum = 0;

    for &num in nums {
        current_sum = (current_sum + num).max(0);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    let nums = [2, 3, 4, 1, 2];
    let max_sum = max_subarray_sum(&nums);
    println!("Maximum subarray sum: {}", max_sum);
}
