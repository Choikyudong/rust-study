fn main() {
    println!("{:?}", roman_to_int(String::from("III")));
    println!("{:?}", roman_to_int(String::from("LVIII")));
    println!("{:?}", roman_to_int(String::from("MCMXCIV")));
}

pub fn roman_to_int(s: String) -> i32 {
    let mut result = 0;
    let mut chars_iter = s.chars().peekable();

    while let Some(current_char) =  chars_iter.next() {
        let current_val = match current_char {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!("invalid roman value {}", current_char),
        };

        if let Some(next_char) = chars_iter.peek() {
            let next_val = match next_char {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => unreachable!("invalid roman value {}", next_char),
            };

            if next_val > current_val {
                result += next_val - current_val;
                chars_iter.next();
                continue;
            }
        }

        result += current_val;
    }

    result
}