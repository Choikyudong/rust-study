fn main() {
    // 13
    println!("{}", score_of_string(String::from("hello")));

    // 50
    println!("{}", score_of_string(String::from("zaz")));
}

fn score_of_string(s: String) -> i32 {
    let mut output = 0;

    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 1 {
        output += (chars[i] as i32 - chars[i + 1] as i32).abs();
    }

    output
}