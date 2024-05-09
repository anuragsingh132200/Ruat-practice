pub fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut curr_sum = nums[0];

    for &num in nums.iter().skip(1) {
        curr_sum = num.max(curr_sum + num);
        max_sum = max_sum.max(curr_sum);
    }

    max_sum
}
