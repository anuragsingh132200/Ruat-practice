pub fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}
