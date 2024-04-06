fn main() {
    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target: i32 = 11;
    let b_search: i32 = search(&arr, target);

    if b_search != -1 {
        println!("Found at index: {}", b_search);
    } else {
        println!("Not found");
    }
}

fn search(arr: &[i32], target: i32) -> i32 {
    let mut low: usize = 0;
    let mut high: usize = arr.len() - 1;
    while low <= high {
        let mid: usize = (low + high) / 2;
        if arr[mid] == target {
            return mid as i32;
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    -1
}
