fn main() {
    println!("{:?}", find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]));
    println!("{:?}" ,find_disappeared_numbers(vec![1, 1]));
}

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut booleans: Vec<bool> = vec![false; nums.len() + 1];

    for &n in nums.iter() {
        booleans[n as usize] = true;
    }

    (1..=nums.len() as i32)
        .filter(|&i| !booleans[i as usize])
        .collect::<Vec<i32>>()
}