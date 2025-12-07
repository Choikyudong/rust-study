fn main() {
    // 4
    println!("{}", min_operations(vec![3, 9, 7], 5));

    // 0
    println!("{}", min_operations(vec![4, 1, 3], 4));
}

fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;

    for i in 0..nums.len() {
        sum += nums[i];
    }

    sum % k
}