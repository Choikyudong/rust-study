fn main() {
    // 1
    println!("{:}", single_number(vec![2,2,1]));

    // 4
    println!("{:}", single_number(vec![4,1,2,1,2]));

    // 1
    println!("{:}", single_number(vec![1]));
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut number = nums[0];

    for i in 1..nums.len() {
        number ^= nums[i];
    }

    number
}