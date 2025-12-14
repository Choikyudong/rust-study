fn main() {
    // [0,0,1,1]
    println!("{:?}", transform_array(vec![4,3,2,1]));

    // [0,0,1,1,1]
    println!("{:?}", transform_array(vec![1,5,1,4,2]));
}

pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
    let total_len = nums.len();
    let mut result = Vec::with_capacity(total_len);

    let mut even_count = 0;
    for num in &nums {
        if num % 2 == 0 {
            result.push(0);
            even_count += 1;
        }
    }

    for _ in even_count..total_len {
        result.push(1);
    }

    result
}

/*
pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());

    for num in nums {
        if num % 2 == 0 {
            result.insert(0, 0);
        } else {
            result.push(1);
        }
    }

    result
}
*/