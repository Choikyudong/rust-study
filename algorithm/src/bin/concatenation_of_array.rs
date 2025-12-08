fn main() {
    // [1,2,1,1,2,1]
    println!("{:?}", get_concatenation(vec![1,2,1]));

    // [1,3,2,1,1,3,2,1]
    println!("{:?}", get_concatenation(vec![1,3,2,1]));
}

fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .chain(nums.iter())
        .cloned()
        .collect()
}