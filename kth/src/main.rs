fn kth_smallest(nums: &[i32], k: usize) -> i32 {
    if k > nums.len() {
        panic!("k is out of range");
    }

    let mut sorted_nums = nums.to_vec();
    sorted_nums.sort();

    sorted_nums[k - 1]
}

fn main() {
    let nums = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    let k = 5;
    let kth_smallest = kth_smallest(&nums, k);
    println!("The {}th smallest element is: {}", k, kth_smallest);
}
