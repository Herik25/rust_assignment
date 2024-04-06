fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> [i32; 8] {
    let mut result = [0; 8]; // Create a new array to hold the merged elements
    let (mut i, mut j, mut k) = (0, 0, 0);

    // Iterate over both arrays simultaneously
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            result[k] = arr1[i];
            i += 1;
        } else {
            result[k] = arr2[j];
            j += 1;
        }
        k += 1;
    }

    // Add remaining elements from arr1
    while i < arr1.len() {
        result[k] = arr1[i];
        i += 1;
        k += 1;
    }

    // Add remaining elements from arr2
    while j < arr2.len() {
        result[k] = arr2[j];
        j += 1;
        k += 1;
    }

    result
}

fn main() {
    let arr1 = [1, 3, 5, 7];
    let arr2 = [2, 4, 6, 8];
    let merged = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}
