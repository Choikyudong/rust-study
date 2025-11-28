use std::i32;


fn main() {
    let output = two_sum(vec![2,7,11,15], 9);
    println!("{:?}",output);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for (i, &current_num) in nums.iter().enumerate() {
        let key = target - current_num;

        if let Some(&j) = map.get(&key) {
            return vec![i as i32, j as i32];
        }

        map.insert(current_num, i);
    }

    vec![]
}
