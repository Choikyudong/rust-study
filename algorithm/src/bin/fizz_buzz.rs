fn main() {
    println!("{:?}", fizz_buzz(3));
    println!("{:?}", fizz_buzz(5));
    println!("{:?}", fizz_buzz(15));
}

/*
pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for i in 1..=n {
        if i % 15 == 0 {
            result.push("FizzBuzz".to_string());
        } else if i % 3 == 0 {
            result.push("Fizz".to_string());
        } else if i % 5 == 0 {
            result.push("Buzz".to_string());
        } else {
            result.push(i.to_string());
        }
    }

    result
}
*/

// 러스트 문법으로 리팩토링
pub fn fizz_buzz(n: i32) -> Vec<String> {
    (1..=n).map(|i| {
        match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => i.to_string(),
        }
    }).collect()
}